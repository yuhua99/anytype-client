use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::Tabled;

use super::{ExtraFields, Icon, PropertyLinkValue};

/// Raw object property shape returned by Anytype.
pub type RawObjectProperty = Value;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ValueEnum, Default)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum ObjectLayout {
    #[default]
    Basic,
    Profile,
    Action,
    Note,
}

impl fmt::Display for ObjectLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Basic => "basic",
            Self::Profile => "profile",
            Self::Action => "action",
            Self::Note => "note",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTypeRef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub name: String,
    #[serde(flatten)]
    pub extra: ExtraFields,
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
    pub layout: ObjectLayout,
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
    pub properties: Vec<RawObjectProperty>,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyLinkValue>,
}

#[derive(Debug, Serialize)]
pub struct UpdateObjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<Icon>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<PropertyLinkValue>,
}
