use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{DataResponse, ObjectType, Template, TemplateResponse, TypeResponse};

impl AnytypeClient {
    pub async fn types(&self, space_id: &str) -> Result<DataResponse<ObjectType>> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/types"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn type_get(&self, space_id: &str, type_id: &str) -> Result<TypeResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/types/{type_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn templates(&self, space_id: &str, type_id: &str) -> Result<DataResponse<Template>> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/types/{type_id}/templates"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn template(
        &self,
        space_id: &str,
        type_id: &str,
        template_id: &str,
    ) -> Result<TemplateResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/types/{type_id}/templates/{template_id}"),
            Option::<&()>::None,
        )
        .await
    }
}
