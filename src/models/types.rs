use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::Tabled;

use super::{Icon, Object};

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
    #[serde(default)]
    #[tabled(skip)]
    pub icon: Option<Icon>,
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
