use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, PropertiesArgs, PropertiesCommand},
    models::{CreatePropertyRequest, CreateTagRequest, UpdatePropertyRequest},
    output::{print_data, print_one},
};

use super::{page_options, parse_json_items, resolve_space};

pub async fn run(
    client: &AnytypeClient,
    args: PropertiesArgs,
    output: &OutputFormat,
) -> Result<()> {
    match args.command {
        PropertiesCommand::List { space, page } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client.properties_page(&id, page_options(page)?).await?.data,
                output,
            )
        }
        PropertiesCommand::Get { space, property_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.property(&id, &property_id).await?.property, output)
        }
        PropertiesCommand::Create {
            space,
            name,
            format,
            key,
            tags,
            tags_json,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = CreatePropertyRequest {
                key,
                name,
                format,
                tags: parse_json_items::<CreateTagRequest>(
                    tags,
                    tags_json,
                    "--tag",
                    "--tags-json",
                )?,
            };
            print_one(client.create_property(&id, &req).await?.property, output)
        }
        PropertiesCommand::Update {
            space,
            property_id,
            name,
            key,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = UpdatePropertyRequest { key, name };
            print_one(
                client
                    .update_property(&id, &property_id, &req)
                    .await?
                    .property,
                output,
            )
        }
        PropertiesCommand::Delete { space, property_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(
                client.delete_property(&id, &property_id).await?.property,
                output,
            )
        }
    }
}
