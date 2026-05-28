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
            &super::space_list_views_path(space_id, list_id),
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
            &super::space_list_view_objects_path(space_id, list_id, view_id),
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
            &super::space_list_objects_path(space_id, list_id),
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
            &super::space_list_object_path(space_id, list_id, object_id),
            Option::<&()>::None,
        )
        .await
    }
}
