use anyhow::{Result, anyhow};
use serde_json::Value;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, SearchArgs},
    models::{SearchRequest, SortOptions},
    output::print_data,
};

use super::{page_options, resolve_space};

pub async fn run(client: &AnytypeClient, args: SearchArgs, output: &OutputFormat) -> Result<()> {
    let page = page_options(args.page)?;
    let req = SearchRequest {
        query: args.query,
        types: args.types,
        filters: parse_filters(args.filters)?,
        sort: args.sort.map(|property_key| SortOptions {
            property_key,
            direction: args.direction,
        }),
    };
    let resp = if let Some(space) = args.space {
        let id = resolve_space(client, &space).await?;
        client.space_search_page(&id, &req, page).await?
    } else {
        client.search_page(&req, page).await?
    };
    print_data(resp.data, output)
}

fn parse_filters(filters: Option<String>) -> Result<Option<Value>> {
    let Some(filters) = filters else {
        return Ok(None);
    };
    let value: Value = serde_json::from_str(&filters)
        .map_err(|err| anyhow!("invalid JSON for --filters: {err}"))?;
    if !value.is_object() {
        return Err(anyhow!("--filters must be a JSON object"));
    }
    Ok(Some(value))
}
