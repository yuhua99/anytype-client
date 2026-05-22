use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{SearchRequest, SearchResponse};

impl AnytypeClient {
    pub async fn search(&self, req: &SearchRequest) -> Result<SearchResponse> {
        self.request_paginated(Method::POST, "/search", Some(req))
            .await
    }

    pub async fn space_search(
        &self,
        space_id: &str,
        req: &SearchRequest,
    ) -> Result<SearchResponse> {
        self.request_paginated(
            Method::POST,
            &format!("/spaces/{space_id}/search"),
            Some(req),
        )
        .await
    }
}
