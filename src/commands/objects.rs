use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::AnytypeClient,
    cli::{ObjectsArgs, ObjectsCommand, OutputFormat},
    models::{CreateObjectRequest, UpdateObjectRequest},
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
