use anyhow::Result;

use crate::{
    api::{AnytypeClient, PageOptions},
    models::{Object, SearchFilters, SearchRequest, SortDirection, SortOptions, SortProperty},
    services::space_resolution::resolve_space,
};

pub(crate) struct SearchParams {
    pub query: String,
    pub types: Vec<String>,
    pub sort: Option<SortProperty>,
    pub direction: SortDirection,
    pub filters: Option<SearchFilters>,
    pub space: Option<String>,
    pub page: Option<PageOptions>,
}

pub(crate) async fn search(client: &AnytypeClient, params: SearchParams) -> Result<Vec<Object>> {
    let req = SearchRequest::new(params.query)
        .with_types(params.types)
        .with_filters(params.filters)
        .with_sort(
            params
                .sort
                .map(|property_key| SortOptions::new(property_key, params.direction)),
        );

    let resp = if let Some(space) = params.space {
        let id = resolve_space(client, &space).await?;
        client.space_search_page(&id, &req, params.page).await?
    } else {
        client.search_page(&req, params.page).await?
    };

    Ok(resp.data)
}
