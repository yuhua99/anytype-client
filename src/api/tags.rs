use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{CreateTagRequest, TagListResponse, TagResponse, UpdateTagRequest};

impl AnytypeClient {
    pub async fn tags(&self, space_id: &str, property_id: &str) -> Result<TagListResponse> {
        self.tags_page(space_id, property_id, None).await
    }

    pub async fn tags_page(
        &self,
        space_id: &str,
        property_id: &str,
        page: Option<PageOptions>,
    ) -> Result<TagListResponse> {
        self.request_data(
            Method::GET,
            &super::space_property_tags_path(space_id, property_id),
            Option::<&()>::None,
            page,
        )
        .await
    }

    pub async fn tag(
        &self,
        space_id: &str,
        property_id: &str,
        tag_id: &str,
    ) -> Result<TagResponse> {
        self.request(
            Method::GET,
            &super::space_property_tag_path(space_id, property_id, tag_id),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn create_tag(
        &self,
        space_id: &str,
        property_id: &str,
        req: &CreateTagRequest,
    ) -> Result<TagResponse> {
        self.request(
            Method::POST,
            &super::space_property_tags_path(space_id, property_id),
            Some(req),
        )
        .await
    }

    pub async fn update_tag(
        &self,
        space_id: &str,
        property_id: &str,
        tag_id: &str,
        req: &UpdateTagRequest,
    ) -> Result<TagResponse> {
        self.request(
            Method::PATCH,
            &super::space_property_tag_path(space_id, property_id, tag_id),
            Some(req),
        )
        .await
    }

    pub async fn delete_tag(
        &self,
        space_id: &str,
        property_id: &str,
        tag_id: &str,
    ) -> Result<TagResponse> {
        self.request(
            Method::DELETE,
            &super::space_property_tag_path(space_id, property_id, tag_id),
            Option::<&()>::None,
        )
        .await
    }
}
