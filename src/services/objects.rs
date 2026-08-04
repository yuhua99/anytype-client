use std::path::PathBuf;

use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::AnytypeClient,
    models::{
        CreateObjectRequest, Icon, Object, Property, PropertyLinkValue, RawObjectProperty,
        SearchRequest, Tag, UpdateObjectRequest,
    },
    services::{
        property_resolution::resolve_property,
        space_resolution::resolve_space,
        tag_resolution::resolve_tag_from_list,
        type_resolution::{resolve_default_template_for_type, resolve_type},
    },
};

mod counts;
mod find;

pub(crate) use counts::{ObjectCountResult, count_objects};
pub(crate) use find::{FindObjectsParams, find_objects};

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
        updated: usize,
        unchanged: usize,
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
    let r#type = resolve_type(client, &space_id, &params.type_key).await?;
    let template_id = if let Some(id) = params.template_id {
        Some(id)
    } else {
        resolve_default_template_for_type(client, &space_id, &r#type).await?
    };
    let req = CreateObjectRequest::new(r#type.key, params.name)
        .with_body(params.body)
        .with_icon(params.icon)
        .with_template_id(template_id)
        .with_properties(params.properties);
    Ok(client.create_object(&space_id, &req).await?.object)
}

pub(crate) async fn update_object(
    client: &AnytypeClient,
    params: UpdateObjectParams,
) -> Result<Object> {
    let space_id = resolve_space(client, &params.space).await?;
    let type_key = match params.type_key {
        Some(type_key) => Some(resolve_type(client, &space_id, &type_key).await?.key),
        None => None,
    };
    let mut req = UpdateObjectRequest::new()
        .with_type_key(type_key)
        .with_name(params.name)
        .with_markdown(params.markdown)
        .with_icon(params.icon)
        .with_properties(params.properties);

    if !params.tag_add.is_empty() || !params.tag_remove.is_empty() {
        let prop_name = params.tag_property.as_deref().ok_or_else(|| {
            anyhow!("--tag-property is required when using --tag-add or --tag-remove")
        })?;
        let property = resolve_property(client, &space_id, prop_name).await?;
        let tag_ids = resolve_tag_ids(
            client,
            &space_id,
            &params.object_id,
            &property,
            &params.tag_add,
            &params.tag_remove,
        )
        .await?;
        req.properties
            .push(PropertyLinkValue::multi_select(&property.key, tag_ids));
    }

    Ok(client
        .update_object(&space_id, &params.object_id, &req)
        .await?
        .object)
}

/// Read current tags from object, merge add/remove, return final tag IDs.
/// `property` must be a resolved property; its canonical `key` is used to
/// read the object's current tags and its `id` to list available tags.
pub(crate) async fn resolve_tag_ids(
    client: &AnytypeClient,
    space_id: &str,
    object_id: &str,
    property: &Property,
    add: &[String],
    remove: &[String],
) -> Result<Vec<String>> {
    let all_tags = client.tags(space_id, &property.id).await?.data;
    let current = get_object_tag_ids(client, space_id, object_id, &property.key).await?;
    merge_tag_ids(current, &all_tags, add, remove)
}

/// Merge tag add/remove requests (by name/key/id) into the current tag IDs.
fn merge_tag_ids(
    mut tag_ids: Vec<String>,
    all_tags: &[Tag],
    add: &[String],
    remove: &[String],
) -> Result<Vec<String>> {
    for name in add {
        let tag_id = resolve_tag_from_list(all_tags, name)?;
        if !tag_ids.contains(&tag_id) {
            tag_ids.push(tag_id);
        }
    }

    for name in remove {
        let tag_id = resolve_tag_from_list(all_tags, name)?;
        tag_ids.retain(|id| id != &tag_id);
    }

    Ok(tag_ids)
}

/// Get current tag IDs from an object's multi-select property by canonical key.
pub(crate) async fn get_object_tag_ids(
    client: &AnytypeClient,
    space_id: &str,
    object_id: &str,
    property_key: &str,
) -> Result<Vec<String>> {
    let object = client.object(space_id, object_id, None).await?.object;
    Ok(tag_ids_from_properties(&object.properties, property_key))
}

fn tag_ids_from_properties(properties: &[RawObjectProperty], property_key: &str) -> Vec<String> {
    properties
        .iter()
        .find(|property| {
            property
                .get("key")
                .and_then(Value::as_str)
                .is_some_and(|key| key.eq_ignore_ascii_case(property_key))
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
        .unwrap_or_default()
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
        let trimmed = id.trim();
        if !trimmed.is_empty() {
            result.push(trimmed.to_string());
        }
    }

    if result.is_empty() {
        let req = SearchRequest::new(query.clone().unwrap_or_default()).with_types(types.to_vec());
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

    let property = if let Some(prop) = prop_name {
        Some(resolve_property(client, &space_id, prop).await?)
    } else {
        None
    };
    let all_tags = if let Some(property) = &property {
        client.tags(&space_id, &property.id).await?.data
    } else {
        Vec::new()
    };

    let mut dry_run_changes = Vec::new();
    let mut updated = 0;

    for object_id in &object_ids {
        let mut req = UpdateObjectRequest::new();
        let mut changes = Vec::new();

        if let Some(property) = &property {
            let current = get_object_tag_ids(client, &space_id, object_id, &property.key).await?;
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
                    .push(PropertyLinkValue::multi_select(&property.key, tag_ids));
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
            updated += 1;
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
            updated,
            unchanged: object_ids.len() - updated,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::IconColor;
    use serde_json::json;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn tag(id: &str, key: &str, name: &str) -> Tag {
        Tag {
            id: id.into(),
            key: key.into(),
            name: name.into(),
            color: IconColor::Yellow,
            object: String::new(),
            extra: Default::default(),
        }
    }

    async fn mock_space_and_type(server: &MockServer) {
        Mock::given(method("GET"))
            .and(path("/v1/spaces"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{"id": "s1", "name": "Work"}]
            })))
            .mount(server)
            .await;
        Mock::given(method("GET"))
            .and(path("/v1/spaces/s1/types"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "data": [{
                    "id": "type-1",
                    "key": "note",
                    "name": "Meeting Note",
                    "default_template_id": "template-1"
                }]
            })))
            .mount(server)
            .await;
    }

    #[tokio::test]
    async fn display_name_type_is_sent_as_resolved_key() {
        let server = MockServer::start().await;
        mock_space_and_type(&server).await;

        Mock::given(method("POST"))
            .and(path("/v1/spaces/s1/objects"))
            .and(body_json(json!({
                "type_key": "note",
                "name": "Created",
                "template_id": "template-1"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "object": {"id": "obj-1", "name": "Created", "space_id": "s1"}
            })))
            .mount(&server)
            .await;
        Mock::given(method("PATCH"))
            .and(path("/v1/spaces/s1/objects/obj-1"))
            .and(body_json(json!({"type_key": "note"})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "object": {"id": "obj-1", "name": "Created", "space_id": "s1"}
            })))
            .mount(&server)
            .await;

        let client = AnytypeClient::new(server.uri(), None).unwrap();
        create_object(
            &client,
            CreateObjectParams {
                space: "Work".into(),
                type_key: "Meeting Note".into(),
                name: "Created".into(),
                body: String::new(),
                icon: None,
                template_id: None,
                properties: Vec::new(),
            },
        )
        .await
        .unwrap();
        update_object(
            &client,
            UpdateObjectParams {
                space: "Work".into(),
                object_id: "obj-1".into(),
                type_key: Some("Meeting Note".into()),
                name: None,
                markdown: None,
                icon: None,
                properties: Vec::new(),
                tag_property: None,
                tag_add: Vec::new(),
                tag_remove: Vec::new(),
            },
        )
        .await
        .unwrap();
    }

    #[test]
    fn merge_tag_ids_adds_removes_and_is_idempotent() {
        let all_tags = vec![tag("id-a", "a", "Tag A"), tag("id-b", "b", "Tag B")];

        let merged = merge_tag_ids(
            vec!["id-a".into()],
            &all_tags,
            &["Tag B".into(), "Tag B".into(), "Tag A".into()],
            &[],
        )
        .unwrap();
        assert_eq!(merged, ["id-a", "id-b"]);

        let merged = merge_tag_ids(
            vec!["id-a".into(), "id-b".into()],
            &all_tags,
            &[],
            &["b".into()],
        )
        .unwrap();
        assert_eq!(merged, ["id-a"]);
    }

    #[test]
    fn merge_tag_ids_errors_on_unknown_tag() {
        let all_tags = vec![tag("id-a", "a", "Tag A")];
        assert!(merge_tag_ids(Vec::new(), &all_tags, &["missing".into()], &[]).is_err());
    }

    #[test]
    fn tag_ids_from_properties_matches_canonical_key_only() {
        let properties = vec![json!({
            "key": "tag",
            "name": "Tag",
            "multi_select": [{"id": "id-a"}, "id-b"]
        })];

        // Canonical key matches (case-insensitively) and reads both shapes.
        assert_eq!(
            tag_ids_from_properties(&properties, "Tag"),
            ["id-a", "id-b"]
        );
        // A non-key string finds nothing: callers must resolve to the key first.
        assert!(tag_ids_from_properties(&properties, "My Tags").is_empty());
    }
}
