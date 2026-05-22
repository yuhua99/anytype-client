use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChallengeResponse {
    pub challenge_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub api_key: String,
}

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
}

#[derive(Debug, Serialize)]
pub struct CreateSpaceRequest {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
    #[serde(default)]
    pub format: Option<String>,
    #[serde(default)]
    pub emoji: Option<String>,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTypeRef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Object {
    #[serde(alias = "ID")]
    pub id: String,
    #[serde(default, alias = "Name")]
    pub name: String,
    #[serde(default, alias = "space_id")]
    pub space_id: String,
    #[serde(default, alias = "type")]
    #[tabled(rename = "type", display = "display_object_type")]
    pub object_type: Option<ObjectTypeRef>,
    #[serde(default, alias = "Layout")]
    pub layout: String,
    #[serde(default, alias = "Archived")]
    pub archived: bool,
    #[serde(default, alias = "Markdown")]
    #[tabled(skip)]
    pub markdown: Option<String>,
    #[serde(default)]
    #[tabled(skip)]
    pub icon: Option<Icon>,
    #[serde(default)]
    #[tabled(skip)]
    pub properties: Vec<Value>,
}

fn display_object_type(object_type: &Option<ObjectTypeRef>) -> String {
    object_type
        .as_ref()
        .map(|r#type| {
            if r#type.key.is_empty() {
                r#type.name.clone()
            } else {
                r#type.key.clone()
            }
        })
        .unwrap_or_default()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectResponse {
    pub object: Object,
}

#[derive(Debug, Serialize)]
pub struct CreateObjectRequest {
    pub type_key: String,
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    pub properties: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub total: Option<i64>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortOptions>,
}

#[derive(Debug, Serialize)]
pub struct SortOptions {
    pub property_key: String,
    pub direction: String,
}

pub type SearchResponse = DataResponse<Object>;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct ObjectType {
    #[serde(alias = "ID")]
    pub id: String,
    #[serde(alias = "Key")]
    pub key: String,
    #[serde(default, alias = "Name")]
    pub name: String,
    #[serde(default, alias = "Layout")]
    pub layout: String,
    #[serde(default)]
    pub plural_name: String,
    #[serde(default, alias = "Description")]
    pub description: String,
    #[serde(default, alias = "is_archived")]
    pub archived: bool,
    #[serde(default, alias = "is_hidden")]
    pub is_hidden: bool,
    #[serde(default, alias = "properties")]
    #[tabled(skip)]
    pub property_definitions: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeResponse {
    pub r#type: ObjectType,
}

pub type Template = Object;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateResponse {
    pub template: Template,
}

pub type ViewListResponse = DataResponse<ListView>;
pub type ObjectListResponse = DataResponse<Object>;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct ListView {
    pub id: String,
    pub name: String,
    pub layout: String,
    #[serde(default)]
    #[tabled(skip)]
    pub filters: Vec<Value>,
    #[serde(default)]
    #[tabled(skip)]
    pub sorts: Vec<Value>,
}

pub type MemberListResponse = DataResponse<Member>;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Member {
    pub id: String,
    pub name: String,
    pub global_name: String,
    pub identity: String,
    pub role: String,
    pub status: String,
    #[serde(default)]
    #[tabled(skip)]
    pub icon: Option<Icon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberResponse {
    pub member: Member,
}
