use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{SearchRequest, SearchResponse};

impl AnytypeClient {
    pub async fn search(&self, req: &SearchRequest) -> Result<SearchResponse> {
        self.search_page(req, None).await
    }

    pub async fn search_page(
        &self,
        req: &SearchRequest,
        page: Option<PageOptions>,
    ) -> Result<SearchResponse> {
        self.request_data(Method::POST, super::global_search_path(), Some(req), page)
            .await
    }

    pub async fn space_search(
        &self,
        space_id: &str,
        req: &SearchRequest,
    ) -> Result<SearchResponse> {
        self.space_search_page(space_id, req, None).await
    }

    pub async fn space_search_page(
        &self,
        space_id: &str,
        req: &SearchRequest,
        page: Option<PageOptions>,
    ) -> Result<SearchResponse> {
        self.request_data(
            Method::POST,
            &super::space_search_path(space_id),
            Some(req),
            page,
        )
        .await
    }
}
