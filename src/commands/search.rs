use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, SearchArgs},
    models::{SearchRequest, SortOptions},
    output::print_data,
};

use super::resolve_space;

pub async fn run(client: &AnytypeClient, args: SearchArgs, output: &OutputFormat) -> Result<()> {
    let req = SearchRequest {
        query: args.query,
        types: args.types,
        sort: args.sort.map(|property_key| SortOptions {
            property_key,
            direction: args.direction,
        }),
    };
    let resp = if let Some(space) = args.space {
        let id = resolve_space(client, &space).await?;
        client.space_search(&id, &req).await?
    } else {
        client.search(&req).await?
    };
    print_data(resp.data, output)
}
