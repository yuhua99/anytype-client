mod auth;
mod files;
mod lists;
mod members;
mod objects;
mod properties;
mod search;
mod spaces;
mod tags;
mod types;

use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::{
    api::{AnytypeClient, PageOptions},
    cli::{Command, IconArgs, OutputFormat, PageArgs, PropertyLinkArgs, PropertyValueArgs},
    models::{Icon, PropertyLink, PropertyLinkValue, Space},
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
        Command::Properties(args) => properties::run(&client, args, &output).await,
        Command::Tags(args) => tags::run(&client, args, &output).await,
        Command::Files(args) => files::run(&client, args, &output).await,
        Command::Lists(args) => lists::run(&client, args, &output).await,
        Command::Members(args) => members::run(&client, args, &output).await,
        Command::Auth(_) => unreachable!(),
    }
}

fn page_options(args: PageArgs) -> Result<Option<PageOptions>> {
    let Some(limit) = args.limit else {
        return Ok(None);
    };
    if limit <= 0 || limit > 1000 {
        return Err(anyhow!("--limit must be between 1 and 1000"));
    }
    if args.offset < 0 {
        return Err(anyhow!("--offset must be >= 0"));
    }
    Ok(Some(PageOptions {
        offset: args.offset,
        limit,
    }))
}

fn build_icon(args: IconArgs) -> Result<Option<Icon>> {
    if args.clear_icon {
        return Err(anyhow!("--clear-icon is only valid for update commands"));
    }
    build_icon_value(args)
}

fn build_patch_icon(args: IconArgs) -> Result<Option<Option<Icon>>> {
    if args.clear_icon {
        if has_icon_value(&args) {
            return Err(anyhow!(
                "--clear-icon cannot be combined with icon value options"
            ));
        }
        return Ok(Some(None));
    }
    build_icon_value(args).map(|icon| icon.map(Some))
}

fn build_icon_value(args: IconArgs) -> Result<Option<Icon>> {
    let variants = [
        args.icon.as_ref().map(|_| "--icon"),
        args.icon_emoji.as_ref().map(|_| "--icon-emoji"),
        args.icon_file.as_ref().map(|_| "--icon-file"),
        args.icon_name.as_ref().map(|_| "--icon-name"),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    if variants.len() > 1 {
        return Err(anyhow!(
            "icon options are mutually exclusive: {}",
            variants.join(", ")
        ));
    }

    Ok(
        match (args.icon, args.icon_emoji, args.icon_file, args.icon_name) {
            (Some(emoji), None, None, None) | (None, Some(emoji), None, None) => {
                Some(Icon::Emoji { emoji })
            }
            (None, None, Some(file), None) => Some(Icon::File { file }),
            (None, None, None, Some(name)) => Some(Icon::Named {
                name,
                color: args.icon_color,
            }),
            (None, None, None, None) => None,
            _ => unreachable!(),
        },
    )
}

fn has_icon_value(args: &IconArgs) -> bool {
    args.icon.is_some()
        || args.icon_emoji.is_some()
        || args.icon_file.is_some()
        || args.icon_name.is_some()
}

fn parse_property_values(args: PropertyValueArgs) -> Result<Vec<PropertyLinkValue>> {
    parse_json_items(
        args.properties,
        args.properties_json,
        "--property",
        "--properties-json",
    )
}

fn parse_property_links(args: PropertyLinkArgs) -> Result<Vec<PropertyLink>> {
    parse_json_items(
        args.properties,
        args.properties_json,
        "--property",
        "--properties-json",
    )
}

fn parse_json_items<T: DeserializeOwned>(
    items: Vec<String>,
    items_json: Option<String>,
    item_arg: &str,
    array_arg: &str,
) -> Result<Vec<T>> {
    let mut parsed = Vec::new();

    if let Some(items_json) = items_json {
        let value = parse_json(&items_json, array_arg)?;
        let array = value
            .as_array()
            .ok_or_else(|| anyhow!("{array_arg} must be a JSON array"))?;
        for value in array {
            parsed.push(parse_json_value(value.clone(), array_arg)?);
        }
    }

    for item in items {
        let value = parse_json(&item, item_arg)?;
        if !value.is_object() {
            return Err(anyhow!("{item_arg} must be a JSON object"));
        }
        parsed.push(parse_json_value(value, item_arg)?);
    }

    Ok(parsed)
}

fn parse_json_value<T: DeserializeOwned>(value: Value, arg: &str) -> Result<T> {
    serde_json::from_value(value).map_err(|err| anyhow!("invalid schema for {arg}: {err}"))
}

fn parse_json(input: &str, arg: &str) -> Result<Value> {
    serde_json::from_str(input).map_err(|err| anyhow!("invalid JSON for {arg}: {err}"))
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
