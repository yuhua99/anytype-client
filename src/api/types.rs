use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{
    CreateTypeRequest, DataResponse, DeleteTypeResponse, ObjectType, Template, TemplateResponse,
    TypeResponse, UpdateTypeRequest,
};

impl AnytypeClient {
    pub async fn types(&self, space_id: &str) -> Result<DataResponse<ObjectType>> {
        self.types_page(space_id, None).await
    }

    pub async fn types_page(
        &self,
        space_id: &str,
        page: Option<PageOptions>,
    ) -> Result<DataResponse<ObjectType>> {
        self.request_data(
            Method::GET,
            &super::space_types_path(space_id),
            Option::<&()>::None,
            page,
        )
        .await
    }

    pub async fn create_type(
        &self,
        space_id: &str,
        req: &CreateTypeRequest,
    ) -> Result<TypeResponse> {
        self.request(Method::POST, &super::space_types_path(space_id), Some(req))
            .await
    }

    pub async fn type_get(&self, space_id: &str, type_id: &str) -> Result<TypeResponse> {
        self.request(
            Method::GET,
            &super::space_type_path(space_id, type_id),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn update_type(
        &self,
        space_id: &str,
        type_id: &str,
        req: &UpdateTypeRequest,
    ) -> Result<TypeResponse> {
        self.request(
            Method::PATCH,
            &super::space_type_path(space_id, type_id),
            Some(req),
        )
        .await
    }

    pub async fn delete_type(&self, space_id: &str, type_id: &str) -> Result<DeleteTypeResponse> {
        self.request(
            Method::DELETE,
            &super::space_type_path(space_id, type_id),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn templates(&self, space_id: &str, type_id: &str) -> Result<DataResponse<Template>> {
        self.templates_page(space_id, type_id, None).await
    }

    pub async fn templates_page(
        &self,
        space_id: &str,
        type_id: &str,
        page: Option<PageOptions>,
    ) -> Result<DataResponse<Template>> {
        self.request_data(
            Method::GET,
            &super::space_type_templates_path(space_id, type_id),
            Option::<&()>::None,
            page,
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
            &super::space_type_template_path(space_id, type_id, template_id),
            Option::<&()>::None,
        )
        .await
    }
}
