use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{CreateObjectRequest, DataResponse, Object, ObjectResponse};

impl AnytypeClient {
    pub async fn objects(&self, space_id: &str) -> Result<DataResponse<Object>> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/objects"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn create_object(
        &self,
        space_id: &str,
        req: &CreateObjectRequest,
    ) -> Result<ObjectResponse> {
        self.request(
            Method::POST,
            &format!("/spaces/{space_id}/objects"),
            Some(req),
        )
        .await
    }

    pub async fn object(&self, space_id: &str, object_id: &str) -> Result<ObjectResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/objects/{object_id}"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn delete_object(&self, space_id: &str, object_id: &str) -> Result<ObjectResponse> {
        self.request(
            Method::DELETE,
            &format!("/spaces/{space_id}/objects/{object_id}"),
            Option::<&()>::None,
        )
        .await
    }
}
