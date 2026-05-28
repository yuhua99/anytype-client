use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{
    CreateApiKeyRequest, CreateApiKeyResponse, CreateChallengeRequest, CreateChallengeResponse,
};

impl AnytypeClient {
    pub async fn create_challenge(&self, app_name: &str) -> Result<CreateChallengeResponse> {
        self.request(
            Method::POST,
            "/auth/challenges",
            Some(&CreateChallengeRequest {
                app_name: app_name.to_string(),
            }),
        )
        .await
    }

    pub async fn create_api_key(
        &self,
        challenge_id: &str,
        code: &str,
    ) -> Result<CreateApiKeyResponse> {
        self.request(
            Method::POST,
            "/auth/api_keys",
            Some(&CreateApiKeyRequest {
                challenge_id: challenge_id.to_string(),
                code: code.to_string(),
            }),
        )
        .await
    }
}
