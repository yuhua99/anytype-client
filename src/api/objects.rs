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
            &format!("/spaces/{space_id}/objects"),
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
            &format!("/spaces/{space_id}/objects"),
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
            Some(format) => format!("/spaces/{space_id}/objects/{object_id}?format={format}"),
            None => format!("/spaces/{space_id}/objects/{object_id}"),
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
            &format!("/spaces/{space_id}/objects/{object_id}"),
            Some(req),
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
