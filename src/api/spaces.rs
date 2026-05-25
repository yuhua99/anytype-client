use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{
    CreateSpaceRequest, CreateSpaceResponse, SpaceListResponse, SpaceResponse, UpdateSpaceRequest,
};

impl AnytypeClient {
    pub async fn spaces(&self) -> Result<SpaceListResponse> {
        self.spaces_page(None).await
    }

    pub async fn spaces_page(&self, page: Option<PageOptions>) -> Result<SpaceListResponse> {
        self.request_data(Method::GET, "/spaces", Option::<&()>::None, page)
            .await
    }

    pub async fn create_space(&self, req: &CreateSpaceRequest) -> Result<CreateSpaceResponse> {
        self.request(Method::POST, "/spaces", Some(req)).await
    }

    pub async fn space(&self, id: &str) -> Result<SpaceResponse> {
        self.request(Method::GET, &format!("/spaces/{id}"), Option::<&()>::None)
            .await
    }

    pub async fn update_space(&self, id: &str, req: &UpdateSpaceRequest) -> Result<SpaceResponse> {
        self.request(Method::PATCH, &format!("/spaces/{id}"), Some(req))
            .await
    }
}
