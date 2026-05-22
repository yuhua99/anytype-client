mod auth;
mod lists;
mod members;
mod objects;
mod search;
mod spaces;
mod types;

use anyhow::{Result, anyhow};

use crate::{
    api::AnytypeClient,
    cli::{Command, OutputFormat},
    models::Space,
};

pub use auth::auth_command;

pub async fn run_command(
    command: Command,
    client: AnytypeClient,
    output: OutputFormat,
) -> Result<()> {
    match command {
        Command::Spaces(args) => spaces::run(&client, args, &output).await,
        Command::Objects(args) => objects::run(&client, args, &output).await,
        Command::Search(args) => search::run(&client, args, &output).await,
        Command::Types(args) => types::run(&client, args, &output).await,
        Command::Lists(args) => lists::run(&client, args, &output).await,
        Command::Members(args) => members::run(&client, args, &output).await,
        Command::Auth(_) => unreachable!(),
    }
}

async fn resolve_space(client: &AnytypeClient, id_or_name: &str) -> Result<String> {
    let spaces = client.spaces().await?.data;
    resolve_space_from_list(&spaces, id_or_name)
}

fn resolve_space_from_list(spaces: &[Space], id_or_name: &str) -> Result<String> {
    if spaces.iter().any(|space| space.id == id_or_name) {
        return Ok(id_or_name.to_string());
    }
    if let Some(space) = spaces
        .iter()
        .find(|space| space.name.eq_ignore_ascii_case(id_or_name))
    {
        return Ok(space.id.clone());
    }

    let needle = id_or_name.to_lowercase();
    let matches: Vec<_> = spaces
        .iter()
        .filter(|space| space.name.to_lowercase().contains(&needle))
        .collect();

    match matches.len() {
        0 => Ok(id_or_name.to_string()),
        1 => Ok(matches[0].id.clone()),
        _ => Err(anyhow!(
            "space not found: multiple spaces matched '{}': {}",
            id_or_name,
            matches
                .iter()
                .map(|s| format!("{} ({})", s.name, s.id))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}
