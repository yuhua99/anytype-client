use anyhow::{Result, anyhow};

use crate::{
    api::AnytypeClient,
    cli::{ObjectsArgs, ObjectsCommand, OutputFormat},
    models::{CreateObjectRequest, UpdateObjectRequest},
    output::{print_data, print_one},
};

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
        } => {
            let id = resolve_space(client, &space).await?;
            let req = UpdateObjectRequest {
                type_key: r#type,
                name,
                markdown,
                icon: build_patch_icon(icon)?,
                properties: parse_property_values(properties)?,
            };
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
