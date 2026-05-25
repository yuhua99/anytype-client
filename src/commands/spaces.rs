use anyhow::{Result, anyhow};

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, SpacesArgs, SpacesCommand},
    models::{CreateSpaceRequest, UpdateSpaceRequest},
    output::{print_data, print_one},
};

use super::{page_options, resolve_space};

pub async fn run(client: &AnytypeClient, args: SpacesArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        SpacesCommand::List { page } => {
            print_data(client.spaces_page(page_options(page)?).await?.data, output)
        }
        SpacesCommand::Get { space } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.space(&id).await?.space, output)
        }
        SpacesCommand::Create { name, description } => print_one(
            client
                .create_space(&CreateSpaceRequest { name, description })
                .await?
                .space,
            output,
        ),
        SpacesCommand::Update {
            space,
            name,
            description,
        } => {
            if name.is_none() && description.is_none() {
                return Err(anyhow!(
                    "at least one of --name or --description is required"
                ));
            }
            let id = resolve_space(client, &space).await?;
            let req = UpdateSpaceRequest { name, description };
            print_one(client.update_space(&id, &req).await?.space, output)
        }
    }
}
