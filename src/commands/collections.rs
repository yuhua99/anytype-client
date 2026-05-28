use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{CollectionsArgs, CollectionsCommand, OutputFormat},
    output::{print_data, print_success},
};

use super::{page_options, resolve_space};

pub async fn run(
    client: &AnytypeClient,
    args: CollectionsArgs,
    output: &OutputFormat,
) -> Result<()> {
    match args.command {
        CollectionsCommand::Views {
            space,
            collection_id,
            page,
        } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client
                    .views_page(&id, &collection_id, page_options(page)?)
                    .await?
                    .data,
                output,
            )
        }
        CollectionsCommand::Objects {
            space,
            collection_id,
            view_id,
            page,
        } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client
                    .view_objects_page(&id, &collection_id, &view_id, page_options(page)?)
                    .await?
                    .data,
                output,
            )
        }
        CollectionsCommand::Add {
            space,
            collection_id,
            object_ids,
        } => {
            let id = resolve_space(client, &space).await?;
            client.add_to_list(&id, &collection_id, &object_ids).await?;
            print_success(format!(
                "Successfully added {} object(s) to collection {}",
                object_ids.len(),
                collection_id
            ));
            Ok(())
        }
        CollectionsCommand::Remove {
            space,
            collection_id,
            object_id,
        } => {
            let id = resolve_space(client, &space).await?;
            client
                .remove_from_list(&id, &collection_id, &object_id)
                .await?;
            print_success(format!(
                "Successfully removed object {} from collection {}",
                object_id, collection_id
            ));
            Ok(())
        }
    }
}
