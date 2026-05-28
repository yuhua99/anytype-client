use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::Tabled;

use super::{ExtraFields, Icon, Object, ObjectLayout, PropertyLink};

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct ObjectType {
    #[serde(alias = "ID")]
    pub id: String,
    #[serde(alias = "Key")]
    pub key: String,
    #[serde(default, alias = "Name")]
    pub name: String,
    #[serde(default, alias = "Layout")]
    pub layout: ObjectLayout,
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
    #[serde(default)]
    #[tabled(skip)]
    pub icon: Option<Icon>,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeResponse {
    pub r#type: ObjectType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTypeResponse {
    pub r#type: Option<ObjectType>,
}

#[derive(Debug, Serialize)]
pub struct CreateTypeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
    pub plural_name: String,
    pub layout: ObjectLayout,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyLink>,
}

#[derive(Debug, Serialize)]
pub struct UpdateTypeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plural_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<ObjectLayout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<Icon>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyLink>,
}

pub type Template = Object;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateResponse {
    pub template: Template,
}
