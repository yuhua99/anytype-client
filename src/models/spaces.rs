use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::{DataResponse, ExtraFields, Icon};

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Space {
    #[serde(alias = "ID")]
    pub id: String,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(default, alias = "Description")]
    pub description: String,
    #[serde(default, alias = "home_id")]
    #[tabled(skip)]
    pub home_id: Option<String>,
    #[serde(default)]
    #[tabled(skip)]
    pub icon: Option<Icon>,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}

pub type SpaceListResponse = DataResponse<Space>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpaceResponse {
    pub space: SpaceDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSpaceResponse {
    pub space: Space,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct SpaceDetail {
    #[serde(alias = "ID")]
    pub id: String,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(default, alias = "Description")]
    pub description: String,
    #[serde(default, alias = "home_id")]
    #[tabled(skip)]
    pub home_id: Option<String>,
    #[serde(default, alias = "archive_id")]
    #[tabled(skip)]
    pub archive_id: Option<String>,
    #[serde(default, alias = "profile_id")]
    #[tabled(skip)]
    pub profile_id: Option<String>,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize)]
pub struct CreateSpaceRequest {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateSpaceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
