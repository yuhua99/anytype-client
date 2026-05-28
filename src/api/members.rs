use anyhow::Result;
use reqwest::Method;

use super::{AnytypeClient, PageOptions};
use crate::models::{MemberListResponse, MemberResponse};

impl AnytypeClient {
    pub async fn members(&self, space_id: &str) -> Result<MemberListResponse> {
        self.members_page(space_id, None).await
    }

    pub async fn members_page(
        &self,
        space_id: &str,
        page: Option<PageOptions>,
    ) -> Result<MemberListResponse> {
        self.request_data(
            Method::GET,
            &super::space_members_path(space_id),
            Option::<&()>::None,
            page,
        )
        .await
    }

    pub async fn member(&self, space_id: &str, member_id: &str) -> Result<MemberResponse> {
        self.request(
            Method::GET,
            &super::space_member_path(space_id, member_id),
            Option::<&()>::None,
        )
        .await
    }
}
