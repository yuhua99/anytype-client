use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{AddToListRequest, ObjectListResponse, ViewListResponse};

impl AnytypeClient {
    pub async fn views(&self, space_id: &str, list_id: &str) -> Result<ViewListResponse> {
        self.views_page(space_id, list_id, None).await
    }

    pub async fn views_page(
        &self,
        space_id: &str,
        list_id: &str,
        page: Option<PageOptions>,
    ) -> Result<ViewListResponse> {
        self.request_data(
            Method::GET,
            &format!("/spaces/{space_id}/lists/{list_id}/views"),
            Option::<&()>::None,
            page,
        )
        .await
    }

    pub async fn view_objects(
        &self,
        space_id: &str,
        list_id: &str,
        view_id: &str,
    ) -> Result<ObjectListResponse> {
        self.view_objects_page(space_id, list_id, view_id, None)
            .await
    }

    pub async fn view_objects_page(
        &self,
        space_id: &str,
        list_id: &str,
        view_id: &str,
        page: Option<PageOptions>,
    ) -> Result<ObjectListResponse> {
        self.request_data(
            Method::GET,
            &format!("/spaces/{space_id}/lists/{list_id}/views/{view_id}/objects"),
            Option::<&()>::None,
            page,
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
            Some(&AddToListRequest { objects }),
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
