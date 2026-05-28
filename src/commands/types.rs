use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, TypesArgs, TypesCommand},
    models::{CreateTypeRequest, UpdateTypeRequest},
    output::{print_data, print_one, print_success},
};

use super::{
    build_icon, build_patch_icon, page_options, property_values::parse_property_links,
    resolve_space,
};

pub async fn run(client: &AnytypeClient, args: TypesArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        TypesCommand::List { space, page } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client.types_page(&id, page_options(page)?).await?.data,
                output,
            )
        }
        TypesCommand::Get { space, type_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.type_get(&id, &type_id).await?.r#type, output)
        }
        TypesCommand::Create {
            space,
            name,
            plural_name,
            layout,
            key,
            icon,
            properties,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = CreateTypeRequest {
                key,
                name,
                plural_name,
                layout,
                icon: build_icon(icon)?,
                properties: parse_property_links(properties)?,
            };
            print_one(client.create_type(&id, &req).await?.r#type, output)
        }
        TypesCommand::Update {
            space,
            type_id,
            name,
            plural_name,
            layout,
            key,
            icon,
            properties,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = UpdateTypeRequest {
                key,
                name,
                plural_name,
                layout,
                icon: build_patch_icon(icon)?,
                properties: parse_property_links(properties)?,
            };
            print_one(
                client.update_type(&id, &type_id, &req).await?.r#type,
                output,
            )
        }
        TypesCommand::Delete { space, type_id } => {
            let id = resolve_space(client, &space).await?;
            match client.delete_type(&id, &type_id).await?.r#type {
                Some(r#type) => print_one(r#type, output),
                None => {
                    print_success("Type deleted");
                    Ok(())
                }
            }
        }
        TypesCommand::Templates {
            space,
            type_id,
            page,
        } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client
                    .templates_page(&id, &type_id, page_options(page)?)
                    .await?
                    .data,
                output,
            )
        }
        TypesCommand::TemplateGet {
            space,
            type_id,
            template_id,
        } => {
            let id = resolve_space(client, &space).await?;
            print_one(
                client.template(&id, &type_id, &template_id).await?.template,
                output,
            )
        }
    }
}
