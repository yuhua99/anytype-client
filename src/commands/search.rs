use anyhow::Result;

use crate::{
    api::AnytypeClient,
    cli::{OutputFormat, SearchArgs},
    output::print_data,
    services::search::{self, SearchParams},
};

use super::page_options;

pub async fn run(client: &AnytypeClient, args: SearchArgs, output: &OutputFormat) -> Result<()> {
    let data = search::search(
        client,
        SearchParams {
            query: args.query,
            types: args.types,
            sort: args.sort,
            direction: args.direction,
            filters: args.filters,
            space: args.space,
            page: page_options(args.page)?,
        },
    )
    .await?;

    print_data(data, output)
}
