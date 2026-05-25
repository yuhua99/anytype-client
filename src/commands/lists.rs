use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{ListsArgs, ListsCommand, OutputFormat},
    output::print_data,
};

use super::{page_options, resolve_space};

pub async fn run(client: &AnytypeClient, args: ListsArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        ListsCommand::Views {
            space,
            list_id,
            page,
        } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client
                    .views_page(&id, &list_id, page_options(page)?)
                    .await?
                    .data,
                output,
            )
        }
        ListsCommand::Objects {
            space,
            list_id,
            view_id,
            page,
        } => {
            let id = resolve_space(client, &space).await?;
            print_data(
                client
                    .view_objects_page(&id, &list_id, &view_id, page_options(page)?)
                    .await?
                    .data,
                output,
            )
        }
        ListsCommand::Add {
            space,
            list_id,
            object_ids,
        } => {
            let id = resolve_space(client, &space).await?;
            client.add_to_list(&id, &list_id, &object_ids).await?;
            println!(
                "Successfully added {} object(s) to list {}",
                object_ids.len(),
                list_id
            );
            Ok(())
        }
        ListsCommand::Remove {
            space,
            list_id,
            object_id,
        } => {
            let id = resolve_space(client, &space).await?;
            client.remove_from_list(&id, &list_id, &object_id).await?;
            println!(
                "Successfully removed object {} from list {}",
                object_id, list_id
            );
            Ok(())
        }
    }
}
