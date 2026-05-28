use crate::{
    api::AnytypeClient,
    cli::{ObjectsArgs, ObjectsCommand, OutputFormat},
    models::{CreateObjectRequest, UpdateObjectRequest},
    output::{print_data, print_one},
    services::{
        objects::{self, FindObjectsParams, ObjectCountResult},
        property_resolution::resolve_property,
        tag_resolution::resolve_tag_from_list,
    },
};
use anyhow::{Result, anyhow};

use super::{build_icon, build_patch_icon, page_options, parse_property_values, resolve_space};

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
                let prop_name = tag_property.as_deref().ok_or_else(|| {
                    anyhow!("--tag-property is required when using --tag-add or --tag-remove")
                })?;
                let tag_ids = objects::resolve_tag_ids(
                    client,
                    &id,
                    &object_id,
                    prop_name,
                    &tag_add,
                    &tag_remove,
                )
                .await?;
                req.properties
                    .push(serde_json::from_value(serde_json::json!({
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
            let object_ids =
                objects::load_object_ids(&ids_file, &ids, &query, &types, client, &id).await?;
            if object_ids.is_empty() {
                eprintln!("no objects matched");
                return Ok(());
            }

            let need_tags = !tag_add.is_empty() || !tag_remove.is_empty();
            let prop_name = if need_tags {
                Some(tag_property.as_deref().ok_or_else(|| {
                    anyhow!("--tag-property is required when using --tag-add or --tag-remove")
                })?)
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
                    let current = objects::get_object_tag_ids(client, &id, oid, prop).await?;
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
            let results = objects::find_objects(
                client,
                FindObjectsParams {
                    space,
                    type_key: r#type,
                    tag,
                    tag_property,
                    property,
                    name,
                    missing_property,
                },
            )
            .await?;

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
            match objects::count_objects(client, space, group_by).await? {
                ObjectCountResult::Total(total) => match output {
                    OutputFormat::Json => println!("{{\"total\": {total}}}"),
                    OutputFormat::Yaml => println!("total: {total}"),
                    OutputFormat::Table => println!("{total}"),
                },
                ObjectCountResult::Grouped { counts, total } => {
                    print_counts(&counts, total, output)?;
                }
            }
            Ok(())
        }
    }
}

/// Print grouped counts respecting output format.
fn print_counts(
    counts: &std::collections::BTreeMap<String, usize>,
    total: usize,
    output: &OutputFormat,
) -> Result<()> {
    match output {
        OutputFormat::Json => {
            let mut map = serde_json::Map::new();
            for (key, count) in counts {
                map.insert(key.clone(), serde_json::Value::Number((*count).into()));
            }
            map.insert("total".to_string(), serde_json::Value::Number(total.into()));
            println!("{}", serde_json::to_string_pretty(&map)?);
        }
        OutputFormat::Yaml => {
            let mut map = serde_json::Map::new();
            for (key, count) in counts {
                map.insert(key.clone(), serde_json::Value::Number((*count).into()));
            }
            map.insert("total".to_string(), serde_json::Value::Number(total.into()));
            println!("{}", serde_yaml::to_string(&map)?);
        }
        OutputFormat::Table => {
            for (name, count) in counts {
                println!("{name}: {count}");
            }
            println!("---");
            println!("total: {total}");
        }
    }
    Ok(())
}
