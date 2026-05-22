use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{ObjectListResponse, ViewListResponse};

impl AnytypeClient {
    pub async fn views(&self, space_id: &str, list_id: &str) -> Result<ViewListResponse> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/lists/{list_id}/views"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn view_objects(
        &self,
        space_id: &str,
        list_id: &str,
        view_id: &str,
    ) -> Result<ObjectListResponse> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/lists/{list_id}/views/{view_id}/objects"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn add_to_list(
        &self,
        space_id: &str,
        list_id: &str,
        objects: &[String],
    ) -> Result<()> {
        self.request_empty(
            Method::POST,
            &format!("/spaces/{space_id}/lists/{list_id}/objects"),
            Some(&serde_json::json!({ "objects": objects })),
        )
        .await
    }

    pub async fn remove_from_list(
        &self,
        space_id: &str,
        list_id: &str,
        object_id: &str,
    ) -> Result<()> {
        self.request_empty(
            Method::DELETE,
            &format!("/spaces/{space_id}/lists/{list_id}/objects/{object_id}"),
            Option::<&()>::None,
        )
        .await
    }
}
