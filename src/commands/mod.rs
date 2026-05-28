mod auth;
mod collections;
mod files;
mod filter_parsing;
mod members;
mod objects;
mod properties;
mod property_values;
mod search;
mod spaces;
mod tags;
mod types;

use crate::{
    api::{AnytypeClient, PageOptions},
    cli::{Command, IconArgs, OutputFormat, PageArgs},
    models::Icon,
};
use anyhow::{Result, anyhow};

pub(crate) use crate::services::space_resolution::resolve_space;

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
        Command::Collections(args) => collections::run(&client, args, &output).await,
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
