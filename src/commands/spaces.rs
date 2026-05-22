use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, SpacesArgs, SpacesCommand},
    models::CreateSpaceRequest,
    output::{print_data, print_one},
};

use super::resolve_space;

pub async fn run(client: &AnytypeClient, args: SpacesArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        SpacesCommand::List => print_data(client.spaces().await?.data, output),
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
    }
}
