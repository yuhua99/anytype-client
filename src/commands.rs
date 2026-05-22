use std::path::PathBuf;

use anyhow::{Result, anyhow};

use crate::{
    DEFAULT_DESKTOP_BASE_URL, DEFAULT_HEADLESS_BASE_URL,
    cli::{
        AuthArgs, AuthCommand, Command, ListsCommand, MembersCommand, ObjectsCommand, OutputFormat,
        SpacesCommand, TypesCommand,
    },
    client::AnytypeClient,
    config::Config,
    models::{CreateObjectRequest, CreateSpaceRequest, Icon, SearchRequest, SortOptions},
    output::{print_data, print_one},
};

pub async fn auth_command(
    args: AuthArgs,
    mut config: Config,
    config_path: PathBuf,
    cli_base_url: Option<String>,
    cli_api_key: Option<String>,
) -> Result<()> {
    match args.command {
        AuthCommand::Desktop { app_name, force } => {
            if config.api_key.is_some() && !force {
                println!("Already authenticated. Use --force to re-authenticate.");
                return Ok(());
            }

            let base_url = cli_base_url.unwrap_or_else(|| DEFAULT_DESKTOP_BASE_URL.to_string());
            let client = AnytypeClient::new(base_url.clone(), None)?;
            let challenge = client.create_challenge(&app_name).await?;
            println!("Desktop auth challenge created. Check Anytype desktop app.");
            println!("Challenge ID: {}", challenge.challenge_id);
            println!("Enter verification code:");

            let mut code = String::new();
            std::io::stdin().read_line(&mut code)?;
            let token = client
                .create_api_key(&challenge.challenge_id, code.trim())
                .await?;

            config.base_url = Some(base_url);
            config.api_key = Some(token.api_key);
            config.save(&config_path)?;
            println!(
                "Desktop authentication successful. Saved credentials to {}",
                config_path.display()
            );
        }
        AuthCommand::Headless { api_key, force } => {
            let new_key = api_key.or(cli_api_key);
            if config.api_key.is_some() && !force && new_key.is_none() {
                println!("Already authenticated. Use --force to overwrite.");
                return Ok(());
            }

            let key = new_key.ok_or_else(|| {
                anyhow!("missing API key: run `anytype auth apikey create <name>`, then `anyclient auth headless --api-key <key>`")
            })?;

            config.base_url =
                Some(cli_base_url.unwrap_or_else(|| DEFAULT_HEADLESS_BASE_URL.to_string()));
            config.api_key = Some(key);
            config.save(&config_path)?;
            println!("Headless API key saved to {}", config_path.display());
        }
    }
    Ok(())
}

pub async fn run_command(
    command: Command,
    client: AnytypeClient,
    output: OutputFormat,
) -> Result<()> {
    match command {
        Command::Spaces(args) => match args.command {
            SpacesCommand::List => print_data(client.spaces().await?.data, &output),
            SpacesCommand::Get { space } => {
                let id = client.resolve_space(&space).await?;
                print_one(client.space(&id).await?.space, &output)
            }
            SpacesCommand::Create { name, description } => print_one(
                client
                    .create_space(&CreateSpaceRequest { name, description })
                    .await?
                    .space,
                &output,
            ),
        },
        Command::Objects(args) => match args.command {
            ObjectsCommand::List { space } => {
                let id = client.resolve_space(&space).await?;
                print_data(client.objects(&id).await?.data, &output)
            }
            ObjectsCommand::Get { space, object_id } => {
                let id = client.resolve_space(&space).await?;
                print_one(client.object(&id, &object_id).await?.object, &output)
            }
            ObjectsCommand::Create {
                space,
                name,
                r#type,
                body,
                icon,
                template,
            } => {
                let id = client.resolve_space(&space).await?;
                let req = CreateObjectRequest {
                    type_key: r#type,
                    name,
                    body,
                    icon: emoji_icon(icon),
                    template_id: template,
                    properties: vec![],
                };
                print_one(client.create_object(&id, &req).await?.object, &output)
            }
            ObjectsCommand::Delete { space, object_id } => {
                let id = client.resolve_space(&space).await?;
                print_one(client.delete_object(&id, &object_id).await?.object, &output)
            }
            ObjectsCommand::Export { space, object_id } => {
                let id = client.resolve_space(&space).await?;
                let obj = client.object(&id, &object_id).await?.object;
                if matches!(output, OutputFormat::Json | OutputFormat::Yaml) {
                    print_one(obj, &output)
                } else {
                    println!("{}", obj.markdown.unwrap_or_default());
                    Ok(())
                }
            }
        },
        Command::Search(args) => {
            let req = SearchRequest {
                query: args.query,
                types: args.types,
                sort: args.sort.map(|property_key| SortOptions {
                    property_key,
                    direction: args.direction,
                }),
            };
            let resp = if let Some(space) = args.space {
                let id = client.resolve_space(&space).await?;
                client.space_search(&id, &req).await?
            } else {
                client.search(&req).await?
            };
            print_data(resp.data, &output)
        }
        Command::Types(args) => match args.command {
            TypesCommand::List { space } => {
                let id = client.resolve_space(&space).await?;
                print_data(client.types(&id).await?.data, &output)
            }
            TypesCommand::Get { space, type_id } => {
                let id = client.resolve_space(&space).await?;
                print_one(client.type_get(&id, &type_id).await?.r#type, &output)
            }
            TypesCommand::Templates { space, type_id } => {
                let id = client.resolve_space(&space).await?;
                print_data(client.templates(&id, &type_id).await?.data, &output)
            }
            TypesCommand::TemplateGet {
                space,
                type_id,
                template_id,
            } => {
                let id = client.resolve_space(&space).await?;
                print_one(
                    client.template(&id, &type_id, &template_id).await?.template,
                    &output,
                )
            }
        },
        Command::Lists(args) => match args.command {
            ListsCommand::Views { space, list_id } => {
                let id = client.resolve_space(&space).await?;
                print_data(client.views(&id, &list_id).await?.data, &output)
            }
            ListsCommand::Objects {
                space,
                list_id,
                view_id,
            } => {
                let id = client.resolve_space(&space).await?;
                print_data(
                    client.view_objects(&id, &list_id, &view_id).await?.data,
                    &output,
                )
            }
            ListsCommand::Add {
                space,
                list_id,
                object_ids,
            } => {
                let id = client.resolve_space(&space).await?;
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
                let id = client.resolve_space(&space).await?;
                client.remove_from_list(&id, &list_id, &object_id).await?;
                println!(
                    "Successfully removed object {} from list {}",
                    object_id, list_id
                );
                Ok(())
            }
        },
        Command::Members(args) => match args.command {
            MembersCommand::List { space } => {
                let id = client.resolve_space(&space).await?;
                print_data(client.members(&id).await?.data, &output)
            }
            MembersCommand::Get { space, member_id } => {
                let id = client.resolve_space(&space).await?;
                print_one(client.member(&id, &member_id).await?.member, &output)
            }
        },
        Command::Auth(_) => unreachable!(),
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
