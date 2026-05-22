use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{CreateSpaceRequest, CreateSpaceResponse, SpaceListResponse, SpaceResponse};

impl AnytypeClient {
    pub async fn spaces(&self) -> Result<SpaceListResponse> {
        self.request_paginated(Method::GET, "/spaces", Option::<&()>::None)
            .await
    }

    pub async fn create_space(&self, req: &CreateSpaceRequest) -> Result<CreateSpaceResponse> {
        self.request(Method::POST, "/spaces", Some(req)).await
    }

    pub async fn space(&self, id: &str) -> Result<SpaceResponse> {
        self.request(Method::GET, &format!("/spaces/{id}"), Option::<&()>::None)
            .await
    }
}
