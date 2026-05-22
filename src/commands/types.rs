use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, TypesArgs, TypesCommand},
    output::{print_data, print_one},
};

use super::resolve_space;

pub async fn run(client: &AnytypeClient, args: TypesArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        TypesCommand::List { space } => {
            let id = resolve_space(client, &space).await?;
            print_data(client.types(&id).await?.data, output)
        }
        TypesCommand::Get { space, type_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.type_get(&id, &type_id).await?.r#type, output)
        }
        TypesCommand::Templates { space, type_id } => {
            let id = resolve_space(client, &space).await?;
            print_data(client.templates(&id, &type_id).await?.data, output)
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
