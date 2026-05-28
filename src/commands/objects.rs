use crate::{
    api::AnytypeClient,
    cli::{ObjectsArgs, ObjectsCommand, OutputFormat},
    output::{print_data, print_one},
    services::objects::{
        self, BulkUpdateParams, BulkUpdateResult, CreateObjectParams, FindObjectsParams,
        ObjectCountResult, UpdateObjectParams,
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
        } => print_one(
            objects::create_object(
                client,
                CreateObjectParams {
                    space,
                    type_key: r#type,
                    name,
                    body,
                    icon: build_icon(icon)?,
                    template_id: template,
                    properties: parse_property_values(properties)?,
                },
            )
            .await?,
            output,
        ),
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
        } => print_one(
            objects::update_object(
                client,
                UpdateObjectParams {
                    space,
                    object_id,
                    type_key: r#type,
                    name,
                    markdown,
                    icon: build_patch_icon(icon)?,
                    properties: parse_property_values(properties)?,
                    tag_property,
                    tag_add,
                    tag_remove,
                },
            )
            .await?,
            output,
        ),
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
            match objects::update_many_objects(
                client,
                BulkUpdateParams {
                    space,
                    ids_file,
                    ids,
                    query,
                    types,
                    tag_property,
                    tag_add,
                    tag_remove,
                    dry_run,
                },
            )
            .await?
            {
                BulkUpdateResult::NoMatches => eprintln!("no objects matched"),
                BulkUpdateResult::Applied { matched } => eprintln!("{matched} objects updated"),
                BulkUpdateResult::DryRun { matched, changes } => {
                    for change in changes {
                        eprintln!("{}: {}", change.name, change.changes.join(" "));
                    }
                    eprintln!("{matched} objects would change (dry run)");
                }
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
