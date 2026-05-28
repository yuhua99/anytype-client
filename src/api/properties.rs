use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{
    CreatePropertyRequest, PropertyListResponse, PropertyResponse, UpdatePropertyRequest,
};

impl AnytypeClient {
    pub async fn properties(&self, space_id: &str) -> Result<PropertyListResponse> {
        self.properties_page(space_id, None).await
    }

    pub async fn properties_page(
        &self,
        space_id: &str,
        page: Option<PageOptions>,
    ) -> Result<PropertyListResponse> {
        self.request_data(
            Method::GET,
            &super::space_properties_path(space_id),
            Option::<&()>::None,
            page,
        )
        .await
    }

    pub async fn property(&self, space_id: &str, property_id: &str) -> Result<PropertyResponse> {
        self.request(
            Method::GET,
            &super::space_property_path(space_id, property_id),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn create_property(
        &self,
        space_id: &str,
        req: &CreatePropertyRequest,
    ) -> Result<PropertyResponse> {
        self.request(
            Method::POST,
            &super::space_properties_path(space_id),
            Some(req),
        )
        .await
    }

    pub async fn update_property(
        &self,
        space_id: &str,
        property_id: &str,
        req: &UpdatePropertyRequest,
    ) -> Result<PropertyResponse> {
        self.request(
            Method::PATCH,
            &super::space_property_path(space_id, property_id),
            Some(req),
        )
        .await
    }

    pub async fn delete_property(
        &self,
        space_id: &str,
        property_id: &str,
    ) -> Result<PropertyResponse> {
        self.request(
            Method::DELETE,
            &super::space_property_path(space_id, property_id),
            Option::<&()>::None,
        )
        .await
    }
}
