use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, TagsArgs, TagsCommand},
    models::{CreateTagRequest, UpdateTagRequest},
    output::{print_data, print_one},
};

use super::{page_options, resolve_space};

pub async fn run(client: &AnytypeClient, args: TagsArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        TagsCommand::List {
            space,
            property_id,
            page,
        } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client
                    .tags_page(&id, &property_id, page_options(page)?)
                    .await?
                    .data,
                output,
            )
        }
        TagsCommand::Get {
            space,
            property_id,
            tag_id,
        } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.tag(&id, &property_id, &tag_id).await?.tag, output)
        }
        TagsCommand::Create {
            space,
            property_id,
            name,
            color,
            key,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = CreateTagRequest { key, name, color };
            print_one(
                client.create_tag(&id, &property_id, &req).await?.tag,
                output,
            )
        }
        TagsCommand::Update {
            space,
            property_id,
            tag_id,
            name,
            color,
            key,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = UpdateTagRequest { key, name, color };
            print_one(
                client
                    .update_tag(&id, &property_id, &tag_id, &req)
                    .await?
                    .tag,
                output,
            )
        }
        TagsCommand::Delete {
            space,
            property_id,
            tag_id,
        } => {
            let id = resolve_space(client, &space).await?;
            print_one(
                client.delete_tag(&id, &property_id, &tag_id).await?.tag,
                output,
            )
        }
    }
}
