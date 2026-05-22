use anyhow::Result;
use reqwest::Method;

use super::AnytypeClient;
use crate::models::{CreateApiKeyResponse, CreateChallengeResponse};

impl AnytypeClient {
    pub async fn create_challenge(&self, app_name: &str) -> Result<CreateChallengeResponse> {
        self.request(
            Method::POST,
            "/auth/challenges",
            Some(&serde_json::json!({ "app_name": app_name })),
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
            Some(&serde_json::json!({ "challenge_id": challenge_id, "code": code })),
        )
        .await
    }
}
