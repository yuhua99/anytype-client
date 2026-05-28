use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CreateChallengeRequest {
    pub app_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub challenge_id: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChallengeResponse {
    pub challenge_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
}
