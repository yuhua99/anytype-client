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
        self.request_data(Method::GET, super::spaces_path(), Option::<&()>::None, page)
            .await
    }

    pub async fn create_space(&self, req: &CreateSpaceRequest) -> Result<CreateSpaceResponse> {
        self.request(Method::POST, super::spaces_path(), Some(req))
            .await
    }

    pub async fn space(&self, id: &str) -> Result<SpaceResponse> {
        self.request(Method::GET, &super::space_path(id), Option::<&()>::None)
            .await
    }

    pub async fn update_space(&self, id: &str, req: &UpdateSpaceRequest) -> Result<SpaceResponse> {
        self.request(Method::PATCH, &super::space_path(id), Some(req))
            .await
    }
}
