use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{
    CreateObjectRequest, DataResponse, Object, ObjectResponse, UpdateObjectRequest,
};

impl AnytypeClient {
    pub async fn objects(&self, space_id: &str) -> Result<DataResponse<Object>> {
        self.objects_page(space_id, None).await
    }

    pub async fn objects_page(
        &self,
        space_id: &str,
        page: Option<PageOptions>,
    ) -> Result<DataResponse<Object>> {
        self.request_data(
            Method::GET,
            &super::space_objects_path(space_id),
            Option::<&()>::None,
            page,
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
            &super::space_objects_path(space_id),
            Some(req),
        )
        .await
    }

    pub async fn object(
        &self,
        space_id: &str,
        object_id: &str,
        format: Option<&str>,
    ) -> Result<ObjectResponse> {
        let path = match format {
            Some(f) => super::space_object_path_with_format(space_id, object_id, f),
            None => super::space_object_path(space_id, object_id),
        };
        self.request(Method::GET, &path, Option::<&()>::None).await
    }

    pub async fn update_object(
        &self,
        space_id: &str,
        object_id: &str,
        req: &UpdateObjectRequest,
    ) -> Result<ObjectResponse> {
        self.request(
            Method::PATCH,
            &super::space_object_path(space_id, object_id),
            Some(req),
        )
        .await
    }

    pub async fn delete_object(&self, space_id: &str, object_id: &str) -> Result<ObjectResponse> {
        self.request(
            Method::DELETE,
            &super::space_object_path(space_id, object_id),
            Option::<&()>::None,
        )
        .await
    }
}
