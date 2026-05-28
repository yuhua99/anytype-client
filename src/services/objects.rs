use std::path::PathBuf;

use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::AnytypeClient,
    models::{
        CreateObjectRequest, Icon, Object, PropertyLinkValue, SearchRequest, UpdateObjectRequest,
    },
    services::{
        property_resolution::resolve_property, space_resolution::resolve_space,
        tag_resolution::resolve_tag_from_list,
    },
};

mod counts;

pub(crate) use counts::{ObjectCountResult, count_objects};

pub(crate) struct FindObjectsParams {
    pub space: String,
    pub type_key: Option<String>,
    pub tag: Option<String>,
    pub tag_property: Option<String>,
    pub property: Option<String>,
    pub name: Option<String>,
    pub missing_property: Option<String>,
}

pub(crate) struct CreateObjectParams {
    pub space: String,
    pub type_key: String,
    pub name: String,
    pub body: String,
    pub icon: Option<Icon>,
    pub template_id: Option<String>,
    pub properties: Vec<PropertyLinkValue>,
}

pub(crate) struct UpdateObjectParams {
    pub space: String,
    pub object_id: String,
    pub type_key: Option<String>,
    pub name: Option<String>,
    pub markdown: Option<String>,
    pub icon: Option<Option<Icon>>,
    pub properties: Vec<PropertyLinkValue>,
    pub tag_property: Option<String>,
    pub tag_add: Vec<String>,
    pub tag_remove: Vec<String>,
}

pub(crate) struct BulkUpdateParams {
    pub space: String,
    pub ids_file: Option<PathBuf>,
    pub ids: Vec<String>,
    pub query: Option<String>,
    pub types: Vec<String>,
    pub tag_property: Option<String>,
    pub tag_add: Vec<String>,
    pub tag_remove: Vec<String>,
    pub dry_run: bool,
}

pub(crate) enum BulkUpdateResult {
    NoMatches,
    Applied {
        matched: usize,
    },
    DryRun {
        matched: usize,
        changes: Vec<BulkUpdateChange>,
    },
}

pub(crate) struct BulkUpdateChange {
    pub name: String,
    pub changes: Vec<String>,
}

pub(crate) async fn create_object(
    client: &AnytypeClient,
    params: CreateObjectParams,
) -> Result<Object> {
    let space_id = resolve_space(client, &params.space).await?;
    let req = CreateObjectRequest {
        type_key: params.type_key,
        name: params.name,
        body: params.body,
        icon: params.icon,
        template_id: params.template_id,
        properties: params.properties,
    };
    Ok(client.create_object(&space_id, &req).await?.object)
}

pub(crate) async fn update_object(
    client: &AnytypeClient,
    params: UpdateObjectParams,
) -> Result<Object> {
    let space_id = resolve_space(client, &params.space).await?;
    let mut req = UpdateObjectRequest {
        type_key: params.type_key,
        name: params.name,
        markdown: params.markdown,
        icon: params.icon,
        properties: params.properties,
    };

    if !params.tag_add.is_empty() || !params.tag_remove.is_empty() {
        let prop_name = params.tag_property.as_deref().ok_or_else(|| {
            anyhow!("--tag-property is required when using --tag-add or --tag-remove")
        })?;
        let tag_ids = resolve_tag_ids(
            client,
            &space_id,
            &params.object_id,
            prop_name,
            &params.tag_add,
            &params.tag_remove,
        )
        .await?;
        req.properties
            .push(serde_json::from_value(serde_json::json!({
                "key": prop_name,
                "multi_select": tag_ids
            }))?);
    }

    Ok(client
        .update_object(&space_id, &params.object_id, &req)
        .await?
        .object)
}

pub(crate) async fn find_objects(
    client: &AnytypeClient,
    params: FindObjectsParams,
) -> Result<Vec<Object>> {
    let space_id = resolve_space(client, &params.space).await?;
    let search_types = params
        .type_key
        .as_ref()
        .map(|r#type| vec![r#type.clone()])
        .unwrap_or_default();
    let req = SearchRequest {
        query: params.name.clone().unwrap_or_default(),
        types: search_types,
        filters: None,
        sort: None,
    };
    let mut results = client.space_search_page(&space_id, &req, None).await?.data;

    if let Some(tag_name) = &params.tag {
        let prop = params
            .tag_property
            .as_deref()
            .ok_or_else(|| anyhow!("--tag-property is required when using --tag"))?;
        let prop_id = resolve_property(client, &space_id, prop).await?;
        let all_tags = client.tags(&space_id, &prop_id).await?.data;
        let target_id = resolve_tag_from_list(&all_tags, tag_name)?;
        results.retain(|obj| has_tag(obj, prop, &target_id));
    }

    if let Some(prop_expr) = &params.property {
        let (key, value) = prop_expr
            .split_once('=')
            .ok_or_else(|| anyhow!("--property must be key=value"))?;
        results.retain(|obj| {
            obj.properties.iter().any(|property| {
                property.get("key").and_then(Value::as_str) == Some(key)
                    && property_matches_value(property, value)
            })
        });
    }

    if let Some(missing_prop) = &params.missing_property {
        results.retain(|obj| {
            !obj.properties.iter().any(|property| {
                property
                    .get("key")
                    .and_then(Value::as_str)
                    .is_some_and(|key| key.eq_ignore_ascii_case(missing_prop))
            })
        });
    }

    Ok(results)
}

/// Read current tags from object, merge add/remove, return final tag IDs.
pub(crate) async fn resolve_tag_ids(
    client: &AnytypeClient,
    space_id: &str,
    object_id: &str,
    property_name_or_key: &str,
    add: &[String],
    remove: &[String],
) -> Result<Vec<String>> {
    let prop_id = resolve_property(client, space_id, property_name_or_key).await?;
    let all_tags = client.tags(space_id, &prop_id).await?.data;

    let mut tag_ids = get_object_tag_ids(client, space_id, object_id, property_name_or_key).await?;

    for name in add {
        let tag_id = resolve_tag_from_list(&all_tags, name)?;
        if !tag_ids.contains(&tag_id) {
            tag_ids.push(tag_id);
        }
    }

    for name in remove {
        let tag_id = resolve_tag_from_list(&all_tags, name)?;
        tag_ids.retain(|id| id != &tag_id);
    }

    Ok(tag_ids)
}

/// Get current tag IDs from an object's multi-select property.
pub(crate) async fn get_object_tag_ids(
    client: &AnytypeClient,
    space_id: &str,
    object_id: &str,
    property_name_or_key: &str,
) -> Result<Vec<String>> {
    let object = client.object(space_id, object_id, None).await?.object;
    Ok(object
        .properties
        .iter()
        .find(|property| {
            property
                .get("key")
                .and_then(Value::as_str)
                .is_some_and(|key| key.eq_ignore_ascii_case(property_name_or_key))
        })
        .and_then(|property| property.get("multi_select"))
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|value| {
                    value
                        .as_str()
                        .map(String::from)
                        .or_else(|| value.get("id").and_then(Value::as_str).map(String::from))
                })
                .collect()
        })
        .unwrap_or_default())
}

/// Collect object IDs from --ids-file, --ids, or search query.
pub(crate) async fn load_object_ids(
    ids_file: &Option<PathBuf>,
    ids: &[String],
    query: &Option<String>,
    types: &[String],
    client: &AnytypeClient,
    space_id: &str,
) -> Result<Vec<String>> {
    let mut result = Vec::new();

    if let Some(path) = ids_file {
        let content = std::fs::read_to_string(path)
            .map_err(|err| anyhow!("failed to read ids file {:?}: {err}", path))?;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                result.push(trimmed.to_string());
            }
        }
    }

    for id in ids {
        for part in id.split(',') {
            let trimmed = part.trim();
            if !trimmed.is_empty() {
                result.push(trimmed.to_string());
            }
        }
    }

    if result.is_empty() {
        let req = SearchRequest {
            query: query.clone().unwrap_or_default(),
            types: types.to_vec(),
            filters: None,
            sort: None,
        };
        let resp = client.space_search_page(space_id, &req, None).await?;
        result = resp.data.into_iter().map(|object| object.id).collect();
    }

    result.sort();
    result.dedup();
    Ok(result)
}

pub(crate) async fn update_many_objects(
    client: &AnytypeClient,
    params: BulkUpdateParams,
) -> Result<BulkUpdateResult> {
    let space_id = resolve_space(client, &params.space).await?;
    let object_ids = load_object_ids(
        &params.ids_file,
        &params.ids,
        &params.query,
        &params.types,
        client,
        &space_id,
    )
    .await?;

    if object_ids.is_empty() {
        return Ok(BulkUpdateResult::NoMatches);
    }

    let need_tags = !params.tag_add.is_empty() || !params.tag_remove.is_empty();
    let prop_name = if need_tags {
        Some(params.tag_property.as_deref().ok_or_else(|| {
            anyhow!("--tag-property is required when using --tag-add or --tag-remove")
        })?)
    } else {
        None
    };

    let all_tags = if let Some(prop) = prop_name {
        let property_id = resolve_property(client, &space_id, prop).await?;
        client.tags(&space_id, &property_id).await?.data
    } else {
        Vec::new()
    };

    let mut dry_run_changes = Vec::new();

    for object_id in &object_ids {
        let mut req = UpdateObjectRequest {
            type_key: None,
            name: None,
            markdown: None,
            icon: None,
            properties: Vec::new(),
        };
        let mut changes = Vec::new();

        if let Some(prop) = prop_name {
            let current = get_object_tag_ids(client, &space_id, object_id, prop).await?;
            let mut tag_ids = current.clone();

            for name in &params.tag_add {
                let tag_id = resolve_tag_from_list(&all_tags, name)?;
                if !tag_ids.contains(&tag_id) {
                    tag_ids.push(tag_id.clone());
                    changes.push(format!("+{name}"));
                }
            }
            for name in &params.tag_remove {
                let tag_id = resolve_tag_from_list(&all_tags, name)?;
                if tag_ids.contains(&tag_id) {
                    tag_ids.retain(|id| id != &tag_id);
                    changes.push(format!("-{name}"));
                }
            }

            if tag_ids != current {
                req.properties
                    .push(serde_json::from_value(serde_json::json!({
                        "key": prop,
                        "multi_select": tag_ids
                    }))?);
            }
        }

        if req.properties.is_empty() {
            continue;
        }

        if params.dry_run {
            let object = client.object(&space_id, object_id, None).await?.object;
            let name = if object.name.is_empty() {
                object_id.clone()
            } else {
                object.name
            };
            dry_run_changes.push(BulkUpdateChange { name, changes });
        } else {
            client.update_object(&space_id, object_id, &req).await?;
        }
    }

    if params.dry_run {
        Ok(BulkUpdateResult::DryRun {
            matched: object_ids.len(),
            changes: dry_run_changes,
        })
    } else {
        Ok(BulkUpdateResult::Applied {
            matched: object_ids.len(),
        })
    }
}

fn has_tag(object: &Object, prop: &str, target_id: &str) -> bool {
    object
        .properties
        .iter()
        .find(|property| {
            property
                .get("key")
                .and_then(Value::as_str)
                .is_some_and(|key| key.eq_ignore_ascii_case(prop))
        })
        .and_then(|property| property.get("multi_select"))
        .and_then(Value::as_array)
        .is_some_and(|arr| {
            arr.iter().any(|value| {
                value.as_str() == Some(target_id)
                    || value.get("id").and_then(Value::as_str) == Some(target_id)
            })
        })
}

/// Check if a property value matches the target string.
fn property_matches_value(prop: &Value, target: &str) -> bool {
    for key in ["text", "number", "select", "url", "email", "phone"] {
        if let Some(value) = prop.get(key) {
            if value
                .as_str()
                .is_some_and(|s| s.eq_ignore_ascii_case(target))
            {
                return true;
            }
            if value
                .as_f64()
                .is_some_and(|number| number.to_string() == target)
            {
                return true;
            }
        }
    }
    if let Some(value) = prop.get("checkbox")
        && value
            .as_bool()
            .is_some_and(|boolean| boolean.to_string() == target)
    {
        return true;
    }
    false
}
