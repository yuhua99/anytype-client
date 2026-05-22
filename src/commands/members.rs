use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{MembersArgs, MembersCommand, OutputFormat},
    output::{print_data, print_one},
};

use super::resolve_space;

pub async fn run(client: &AnytypeClient, args: MembersArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        MembersCommand::List { space } => {
            let id = resolve_space(client, &space).await?;
            print_data(client.members(&id).await?.data, output)
        }
        MembersCommand::Get { space, member_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.member(&id, &member_id).await?.member, output)
        }
    }
}
