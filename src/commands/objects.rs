use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{ObjectsArgs, ObjectsCommand, OutputFormat},
    models::{CreateObjectRequest, Icon},
    output::{print_data, print_one},
};

use super::resolve_space;

pub async fn run(client: &AnytypeClient, args: ObjectsArgs, output: &OutputFormat) -> Result<()> {
    match args.command {
        ObjectsCommand::List { space } => {
            let id = resolve_space(client, &space).await?;
            print_data(client.objects(&id).await?.data, output)
        }
        ObjectsCommand::Get { space, object_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.object(&id, &object_id).await?.object, output)
        }
        ObjectsCommand::Create {
            space,
            name,
            r#type,
            body,
            icon,
            template,
        } => {
            let id = resolve_space(client, &space).await?;
            let req = CreateObjectRequest {
                type_key: r#type,
                name,
                body,
                icon: emoji_icon(icon),
                template_id: template,
                properties: vec![],
            };
            print_one(client.create_object(&id, &req).await?.object, output)
        }
        ObjectsCommand::Delete { space, object_id } => {
            let id = resolve_space(client, &space).await?;
            print_one(client.delete_object(&id, &object_id).await?.object, output)
        }
        ObjectsCommand::Export { space, object_id } => {
            let id = resolve_space(client, &space).await?;
            let obj = client.object(&id, &object_id).await?.object;
            if matches!(output, OutputFormat::Json | OutputFormat::Yaml) {
                print_one(obj, output)
            } else {
                println!("{}", obj.markdown.unwrap_or_default());
                Ok(())
            }
        }
    }
}

fn emoji_icon(icon: Option<String>) -> Option<Icon> {
    icon.map(|emoji| Icon {
        format: Some("emoji".into()),
        emoji: Some(emoji),
        file: None,
        name: None,
        color: None,
    })
}
