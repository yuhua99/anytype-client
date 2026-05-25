use serde::Serialize;
use serde_json::Value;

use super::{DataResponse, Object};

#[derive(Debug, Serialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<SortOptions>,
}

#[derive(Debug, Serialize)]
pub struct SortOptions {
    pub property_key: String,
    pub direction: String,
}

pub type SearchResponse = DataResponse<Object>;
