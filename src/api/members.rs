use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{MemberListResponse, MemberResponse};

impl AnytypeClient {
    pub async fn members(&self, space_id: &str) -> Result<MemberListResponse> {
        self.request_paginated(
            Method::GET,
            &format!("/spaces/{space_id}/members"),
            Option::<&()>::None,
        )
        .await
    }

    pub async fn member(&self, space_id: &str, member_id: &str) -> Result<MemberResponse> {
        self.request(
            Method::GET,
            &format!("/spaces/{space_id}/members/{member_id}"),
            Option::<&()>::None,
        )
        .await
    }
}
