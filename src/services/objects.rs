use std::collections::BTreeMap;

use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::AnytypeClient,
    models::{Object, SearchRequest, Space, Tag},
};

pub struct FindObjectsParams {
    pub space: String,
    pub type_key: Option<String>,
    pub tag: Option<String>,
    pub tag_property: Option<String>,
    pub property: Option<String>,
    pub name: Option<String>,
    pub missing_property: Option<String>,
}

pub enum ObjectCountResult {
    Total(usize),
    Grouped {
        counts: BTreeMap<String, usize>,
        total: usize,
    },
}

pub async fn find_objects(
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

pub async fn count_objects(
    client: &AnytypeClient,
    space: String,
    group_by: Option<String>,
) -> Result<ObjectCountResult> {
    let space_id = resolve_space(client, &space).await?;
    let req = SearchRequest {
        query: String::new(),
        types: Vec::new(),
        filters: None,
        sort: None,
    };
    let results = client.space_search_page(&space_id, &req, None).await?.data;
    let total = results.len();

    match group_by.as_deref() {
        Some("type") => Ok(ObjectCountResult::Grouped {
            counts: count_by_type(&results),
            total,
        }),
        Some(group) if group.starts_with("property:") => Ok(ObjectCountResult::Grouped {
            counts: count_by_property(&results, &group["property:".len()..]),
            total,
        }),
        Some(other) => Err(anyhow!(
            "invalid --group-by: '{other}'. Use 'type' or 'property:<key>'"
        )),
        None => Ok(ObjectCountResult::Total(total)),
    }
}

fn count_by_type(objects: &[Object]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for object in objects {
        let type_name = object
            .object_type
            .as_ref()
            .map(|object_type| {
                if object_type.key.is_empty() {
                    object_type.name.clone()
                } else {
                    object_type.key.clone()
                }
            })
            .unwrap_or_else(|| "(none)".to_string());
        *counts.entry(type_name).or_insert(0) += 1;
    }
    counts
}

fn count_by_property(objects: &[Object], prop_key: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    let mut missing = 0usize;

    for object in objects {
        let found = object.properties.iter().find(|property| {
            property
                .get("key")
                .and_then(Value::as_str)
                .is_some_and(|key| key.eq_ignore_ascii_case(prop_key))
        });
        match found {
            None => missing += 1,
            Some(property) => {
                let value = display_property_value(property);
                if value.is_empty() {
                    *counts.entry("(empty)".to_string()).or_insert(0) += 1;
                } else {
                    *counts.entry(value).or_insert(0) += 1;
                }
            }
        }
    }

    if missing > 0 {
        counts.insert("(missing)".to_string(), missing);
    }

    counts
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

/// Format a property value for count grouping.
fn display_property_value(prop: &Value) -> String {
    for key in ["text", "url", "email", "phone"] {
        if let Some(value) = prop.get(key).and_then(Value::as_str) {
            return value.to_string();
        }
    }
    if let Some(value) = prop.get("number") {
        return value.to_string();
    }
    if let Some(value) = prop.get("select").and_then(Value::as_str) {
        return value.to_string();
    }
    if let Some(arr) = prop.get("multi_select").and_then(Value::as_array) {
        let names: Vec<String> = arr
            .iter()
            .map(|value| {
                value
                    .get("name")
                    .and_then(Value::as_str)
                    .or_else(|| value.as_str())
                    .unwrap_or("?")
                    .to_string()
            })
            .collect();
        return names.join(", ");
    }
    if let Some(value) = prop.get("checkbox").and_then(Value::as_bool) {
        return value.to_string();
    }
    if let Some(value) = prop.get("date").and_then(Value::as_str) {
        return value.to_string();
    }
    serde_json::to_string(prop).unwrap_or_default()
}

async fn resolve_space(client: &AnytypeClient, id_or_name: &str) -> Result<String> {
    let spaces = client.spaces().await?.data;
    resolve_space_from_list(&spaces, id_or_name)
}

fn resolve_space_from_list(spaces: &[Space], id_or_name: &str) -> Result<String> {
    if spaces.iter().any(|space| space.id == id_or_name) {
        return Ok(id_or_name.to_string());
    }
    if let Some(space) = spaces
        .iter()
        .find(|space| space.name.eq_ignore_ascii_case(id_or_name))
    {
        return Ok(space.id.clone());
    }

    let needle = id_or_name.to_lowercase();
    let matches: Vec<_> = spaces
        .iter()
        .filter(|space| space.name.to_lowercase().contains(&needle))
        .collect();

    match matches.len() {
        0 => Ok(id_or_name.to_string()),
        1 => Ok(matches[0].id.clone()),
        _ => Err(anyhow!(
            "space not found: multiple spaces matched '{}': {}",
            id_or_name,
            matches
                .iter()
                .map(|space| format!("{} ({})", space.name, space.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

async fn resolve_property(
    client: &AnytypeClient,
    space_id: &str,
    name_or_key: &str,
) -> Result<String> {
    let properties = client.properties(space_id).await?.data;
    if let Some(property) = properties
        .iter()
        .find(|property| property.id == name_or_key)
    {
        return Ok(property.id.clone());
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(property.id.clone());
    }
    if let Some(property) = properties
        .iter()
        .find(|property| property.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(property.id.clone());
    }
    let needle = name_or_key.to_lowercase();
    let matches: Vec<_> = properties
        .iter()
        .filter(|property| {
            property.name.to_lowercase().contains(&needle)
                || property.key.to_lowercase().contains(&needle)
        })
        .collect();
    match matches.len() {
        0 => Err(anyhow!(
            "property not found: '{name_or_key}' matched no property name or key"
        )),
        1 => Ok(matches[0].id.clone()),
        _ => Err(anyhow!(
            "property ambiguous: '{name_or_key}' matched multiple: {}",
            matches
                .iter()
                .map(|property| format!("{} ({})", property.name, property.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn resolve_tag_from_list(tags: &[Tag], name_or_key: &str) -> Result<String> {
    if let Some(tag) = tags.iter().find(|tag| tag.id == name_or_key) {
        return Ok(tag.id.clone());
    }
    if let Some(tag) = tags
        .iter()
        .find(|tag| tag.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(tag.id.clone());
    }
    if let Some(tag) = tags
        .iter()
        .find(|tag| tag.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(tag.id.clone());
    }
    let needle = name_or_key.to_lowercase();
    let matches: Vec<_> = tags
        .iter()
        .filter(|tag| {
            tag.name.to_lowercase().contains(&needle) || tag.key.to_lowercase().contains(&needle)
        })
        .collect();
    match matches.len() {
        0 => Err(anyhow!(
            "tag not found: '{name_or_key}' matched no tag name or key"
        )),
        1 => Ok(matches[0].id.clone()),
        _ => Err(anyhow!(
            "tag ambiguous: '{name_or_key}' matched multiple: {}",
            matches
                .iter()
                .map(|tag| format!("{} ({})", tag.name, tag.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
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
