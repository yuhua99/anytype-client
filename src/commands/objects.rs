use std::path::PathBuf;

use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::AnytypeClient,
    cli::{ObjectsArgs, ObjectsCommand, OutputFormat},
    models::{CreateObjectRequest, SearchRequest, UpdateObjectRequest},
    output::{print_data, print_one},
};

use super::{build_icon, build_patch_icon, page_options, parse_property_values, resolve_property, resolve_space};

pub async fn run(client: &AnytypeClient, args: ObjectsArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        ObjectsCommand::List { space, page } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client.objects_page(&id, page_options(page)?).await?.data,
                output,
            )
        }
        ObjectsCommand::Get {
            space,
            object_id,
            format,
        } => {
            let id = resolve_space(client, &space).await?;
            print_one(
                client.object(&id, &object_id, Some(&format)).await?.object,
                output,
            )
        }
        ObjectsCommand::Create {
            space,
            name,
            r#type,
            body,
            icon,
            template,
            properties,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = CreateObjectRequest {
                type_key: r#type,
                name,
                body,
                icon: build_icon(icon)?,
                template_id: template,
                properties: parse_property_values(properties)?,
            };
            print_one(client.create_object(&id, &req).await?.object, output)
        }
        ObjectsCommand::Update {
            space,
            object_id,
            name,
            r#type,
            markdown,
            icon,
            properties,
            tag_property,
            tag_add,
            tag_remove,
        } => {
            let id = resolve_space(client, &space).await?;
            let mut req = UpdateObjectRequest {
                type_key: r#type,
                name,
                markdown,
                icon: build_patch_icon(icon)?,
                properties: parse_property_values(properties)?,
            };
            if !tag_add.is_empty() || !tag_remove.is_empty() {
                let prop_name = tag_property
                    .as_deref()
                    .ok_or_else(|| anyhow!("--tag-property is required when using --tag-add or --tag-remove"))?;
                let tag_ids = resolve_tag_ids(
                    client,
                    &id,
                    &object_id,
                    prop_name,
                    &tag_add,
                    &tag_remove,
                )
                .await?;
                req.properties.push(serde_json::from_value(serde_json::json!({
                    "key": prop_name,
                    "multi_select": tag_ids
                }))?);
            }
            print_one(
                client.update_object(&id, &object_id, &req).await?.object,
                output,
            )
        }
        ObjectsCommand::Delete { space, object_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.delete_object(&id, &object_id).await?.object, output)
        }
        ObjectsCommand::Export {
            space,
            object_id,
            format,
        } => {
            let id = resolve_space(client, &space).await?;
            let obj = client.object(&id, &object_id, Some(&format)).await?.object;
            if matches!(output, OutputFormat::Json | OutputFormat::Yaml) {
                print_one(obj, output)
            } else {
                let markdown = obj.markdown.ok_or_else(|| {
                    anyhow!("object response did not include markdown body for format {format}")
                })?;
                println!("{markdown}");
                Ok(())
            }
        }
        ObjectsCommand::UpdateMany {
            space,
            ids_file,
            ids,
            query,
            types,
            tag_property,
            tag_add,
            tag_remove,
            dry_run,
        } => {
            let id = resolve_space(client, &space).await?;

            // Collect target object IDs
            let object_ids = load_object_ids(&ids_file, &ids, &query, &types, client, &id).await?;
            if object_ids.is_empty() {
                eprintln!("no objects matched");
                return Ok(());
            }

            let need_tags = !tag_add.is_empty() || !tag_remove.is_empty();
            let prop_name = if need_tags {
                Some(
                    tag_property
                        .as_deref()
                        .ok_or_else(|| anyhow!("--tag-property is required when using --tag-add or --tag-remove"))?,
                )
            } else {
                None
            };

            // Resolve tag list once if needed
            let (_prop_id, all_tags) = if let Some(prop) = prop_name {
                let pid = resolve_property(client, &id, prop).await?;
                let tags = client.tags(&id, &pid).await?.data;
                (Some(pid), tags)
            } else {
                (None, Vec::new())
            };

            for oid in &object_ids {
                let mut req = UpdateObjectRequest {
                    type_key: None,
                    name: None,
                    markdown: None,
                    icon: None,
                    properties: Vec::new(),
                };

                let mut changes = Vec::new();

                if need_tags {
                    let prop = prop_name.unwrap();
                    let current = get_object_tag_ids(client, &id, oid, prop).await?;
                    let mut tag_ids = current.clone();

                    for name in &tag_add {
                        let tag_id = resolve_tag_from_list(&all_tags, name)?;
                        if !tag_ids.contains(&tag_id) {
                            tag_ids.push(tag_id.clone());
                            changes.push(format!("+{name}"));
                        }
                    }
                    for name in &tag_remove {
                        let tag_id = resolve_tag_from_list(&all_tags, name)?;
                        if tag_ids.contains(&tag_id) {
                            tag_ids.retain(|i| i != &tag_id);
                            changes.push(format!("-{name}"));
                        }
                    }

                    if tag_ids != current {
                        req.properties.push(serde_json::from_value(serde_json::json!({
                            "key": prop,
                            "multi_select": tag_ids
                        }))?);
                    }
                }

                if req.properties.is_empty() {
                    continue;
                }

                if dry_run {
                    let obj = client.object(&id, oid, None).await?.object;
                    let name = if obj.name.is_empty() {
                        oid.as_str()
                    } else {
                        &obj.name
                    };
                    eprintln!("{name}: {}", changes.join(" "));
                } else {
                    client.update_object(&id, oid, &req).await?;
                }
            }

            if dry_run {
                eprintln!("{} objects would change (dry run)", object_ids.len());
            } else {
                eprintln!("{} objects updated", object_ids.len());
            }
            Ok(())
        }
        ObjectsCommand::Find {
            space,
            r#type,
            tag,
            tag_property,
            property,
            name,
            missing_property,
            ids_only,
            names_only,
        } => {
            let id = resolve_space(client, &space).await?;

            // Search for all objects in space
            let search_types = r#type
                .as_ref()
                .map(|t| vec![t.clone()])
                .unwrap_or_default();
            let req = SearchRequest {
                query: name.clone().unwrap_or_default(),
                types: search_types,
                filters: None,
                sort: None,
            };
            let mut results = client.space_search_page(&id, &req, None).await?.data;

            // Post-filter by tag
            if let Some(tag_name) = &tag {
                let prop = tag_property
                    .as_deref()
                    .ok_or_else(|| anyhow!("--tag-property is required when using --tag"))?;
                let prop_id = resolve_property(client, &id, prop).await?;
                let all_tags = client.tags(&id, &prop_id).await?.data;
                let target_id = resolve_tag_from_list(&all_tags, tag_name)?;
                results.retain(|obj| {
                    obj.properties
                        .iter()
                        .find(|p| {
                            p.get("key")
                                .and_then(Value::as_str)
                                .map_or(false, |k| k.eq_ignore_ascii_case(prop))
                        })
                        .and_then(|p| p.get("multi_select"))
                        .and_then(Value::as_array)
                        .map_or(false, |arr| {
                            arr.iter().any(|v| {
                                v.as_str() == Some(&target_id)
                                    || v.get("id").and_then(Value::as_str) == Some(&target_id)
                            })
                        })
                });
            }

            // Post-filter by property value
            if let Some(prop_expr) = &property {
                let (key, value) = prop_expr
                    .split_once('=')
                    .ok_or_else(|| anyhow!("--property must be key=value"))?;
                results.retain(|obj| {
                    obj.properties.iter().any(|p| {
                        p.get("key").and_then(Value::as_str) == Some(key)
                            && property_matches_value(p, value)
                    })
                });
            }

            // Post-filter: missing property
            if let Some(missing_prop) = &missing_property {
                results.retain(|obj| {
                    !obj.properties.iter().any(|p| {
                        p.get("key")
                            .and_then(Value::as_str)
                            .map_or(false, |k| k.eq_ignore_ascii_case(missing_prop))
                    })
                });
            }

            // Output
            if ids_only {
                for obj in &results {
                    println!("{}", obj.id);
                }
            } else if names_only {
                for obj in &results {
                    println!("{}", obj.name);
                }
            } else {
                print_data(results, output)?;
            }
            Ok(())
        }
        ObjectsCommand::Count { space, group_by } => {
            let id = resolve_space(client, &space).await?;
            let req = SearchRequest {
                query: String::new(),
                types: Vec::new(),
                filters: None,
                sort: None,
            };
            let results = client.space_search_page(&id, &req, None).await?.data;

            match group_by.as_deref() {
                Some("type") => {
                    let mut counts: std::collections::BTreeMap<String, usize> =
                        std::collections::BTreeMap::new();
                    for obj in &results {
                        let type_name = obj
                            .object_type
                            .as_ref()
                            .map(|t| if t.key.is_empty() { t.name.clone() } else { t.key.clone() })
                            .unwrap_or_else(|| "(none)".to_string());
                        *counts.entry(type_name).or_insert(0) += 1;
                    }
                    for (name, count) in &counts {
                        println!("{name}: {count}");
                    }
                    println!("---");
                    println!("total: {}", results.len());
                }
                Some(group) if group.starts_with("property:") => {
                    let prop_key = &group["property:".len()..];
                    let mut counts: std::collections::BTreeMap<String, usize> =
                        std::collections::BTreeMap::new();
                    let mut missing = 0usize;
                    for obj in &results {
                        let val = obj
                            .properties
                            .iter()
                            .find(|p| {
                                p.get("key")
                                    .and_then(Value::as_str)
                                    .map_or(false, |k| k.eq_ignore_ascii_case(prop_key))
                            })
                            .map(display_property_value)
                            .unwrap_or_default();
                        if val.is_empty() {
                            missing += 1;
                        } else {
                            *counts.entry(val).or_insert(0) += 1;
                        }
                    }
                    for (val, count) in &counts {
                        println!("{val}: {count}");
                    }
                    if missing > 0 {
                        println!("(missing): {missing}");
                    }
                    println!("---");
                    println!("total: {}", results.len());
                }
                Some(other) => {
                    return Err(anyhow!(
                        "invalid --group-by: '{other}'. Use 'type' or 'property:<key>'"
                    ));
                }
                None => {
                    println!("{}", results.len());
                }
            }
            Ok(())
        }
    }
}

/// Format a property value for human-readable display.
fn display_property_value(prop: &Value) -> String {
    for key in ["text", "url", "email", "phone"] {
        if let Some(val) = prop.get(key).and_then(Value::as_str) {
            return val.to_string();
        }
    }
    if let Some(val) = prop.get("number") {
        return val.to_string();
    }
    if let Some(val) = prop.get("select").and_then(Value::as_str) {
        return val.to_string();
    }
    if let Some(arr) = prop.get("multi_select").and_then(Value::as_array) {
        let names: Vec<String> = arr
            .iter()
            .map(|v| {
                v.get("name")
                    .and_then(Value::as_str)
                    .or_else(|| v.as_str())
                    .unwrap_or("?")
                    .to_string()
            })
            .collect();
        return names.join(", ");
    }
    if let Some(val) = prop.get("checkbox").and_then(Value::as_bool) {
        return val.to_string();
    }
    if let Some(val) = prop.get("date").and_then(Value::as_str) {
        return val.to_string();
    }
    // fallback: raw JSON
    serde_json::to_string(prop).unwrap_or_default()
}

/// Print object properties in human-readable format.
pub fn print_properties(props: &[Value]) {
    for prop in props {
        let key = prop
            .get("key")
            .and_then(Value::as_str)
            .unwrap_or("?");
        let name = prop
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or(key);
        let value = display_property_value(prop);
        println!("  {name}: {value}");
    }
}

/// Read current tags from object, merge add/remove, return final tag IDs.
async fn resolve_tag_ids(
    client: &AnytypeClient,
    space_id: &str,
    object_id: &str,
    property_name_or_key: &str,
    add: &[String],
    remove: &[String],
) -> Result<Vec<String>> {
    let prop_id = resolve_property(client, space_id, property_name_or_key).await?;
    let all_tags = client.tags(space_id, &prop_id).await?.data;

    // Get current tag IDs from the object
    let obj = client.object(space_id, object_id, None).await?.object;
    let mut tag_ids: Vec<String> = obj
        .properties
        .iter()
        .find(|p| {
            p.get("key")
                .and_then(Value::as_str)
                .map_or(false, |k| k.eq_ignore_ascii_case(property_name_or_key))
        })
        .and_then(|p| p.get("multi_select"))
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    // Handle both string IDs and tag objects
                    v.as_str().map(String::from).or_else(|| {
                        v.get("id").and_then(Value::as_str).map(String::from)
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    // Resolve and add new tags
    for name in add {
        let tag_id = resolve_tag_from_list(&all_tags, name)?;
        if !tag_ids.contains(&tag_id) {
            tag_ids.push(tag_id);
        }
    }

    // Resolve and remove tags
    for name in remove {
        let tag_id = resolve_tag_from_list(&all_tags, name)?;
        tag_ids.retain(|id| id != &tag_id);
    }

    Ok(tag_ids)
}

fn resolve_tag_from_list(
    tags: &[crate::models::Tag],
    name_or_key: &str,
) -> Result<String> {
    if let Some(t) = tags.iter().find(|t| t.id == name_or_key) {
        return Ok(t.id.clone());
    }
    if let Some(t) = tags
        .iter()
        .find(|t| t.key.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(t.id.clone());
    }
    if let Some(t) = tags
        .iter()
        .find(|t| t.name.eq_ignore_ascii_case(name_or_key))
    {
        return Ok(t.id.clone());
    }
    let needle = name_or_key.to_lowercase();
    let matches: Vec<_> = tags
        .iter()
        .filter(|t| {
            t.name.to_lowercase().contains(&needle) || t.key.to_lowercase().contains(&needle)
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
                .map(|t| format!("{} ({})", t.name, t.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

/// Collect object IDs from --ids-file, --ids, or search query.
async fn load_object_ids(
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
            .map_err(|e| anyhow!("failed to read ids file {:?}: {e}", path))?;
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
        result = resp.data.into_iter().map(|o| o.id).collect();
    }

    result.sort();
    result.dedup();
    Ok(result)
}

/// Get current tag IDs from an object's multi-select property.
async fn get_object_tag_ids(
    client: &AnytypeClient,
    space_id: &str,
    object_id: &str,
    property_name_or_key: &str,
) -> Result<Vec<String>> {
    let obj = client.object(space_id, object_id, None).await?.object;
    Ok(obj
        .properties
        .iter()
        .find(|p| {
            p.get("key")
                .and_then(Value::as_str)
                .map_or(false, |k| k.eq_ignore_ascii_case(property_name_or_key))
        })
        .and_then(|p| p.get("multi_select"))
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|v| {
                    v.as_str().map(String::from).or_else(|| {
                        v.get("id").and_then(Value::as_str).map(String::from)
                    })
                })
                .collect()
        })
        .unwrap_or_default())
}

/// Check if a property value matches the target string.
fn property_matches_value(prop: &Value, target: &str) -> bool {
    // Try common value fields
    for key in ["text", "number", "select", "url", "email", "phone"] {
        if let Some(val) = prop.get(key) {
            if val.as_str().map_or(false, |s| s.eq_ignore_ascii_case(target)) {
                return true;
            }
            if val.as_f64().map_or(false, |n| n.to_string() == target) {
                return true;
            }
        }
    }
    // checkbox
    if let Some(val) = prop.get("checkbox") {
        if val.as_bool().map_or(false, |b| b.to_string() == target) {
            return true;
        }
    }
    false
}
