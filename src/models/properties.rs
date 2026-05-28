use std::fmt;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::{DataResponse, ExtraFields, IconColor};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum PropertyFormat {
    Text,
    Number,
    Select,
    MultiSelect,
    Date,
    Files,
    Checkbox,
    Url,
    Email,
    Phone,
    Objects,
}

impl fmt::Display for PropertyFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Text => "text",
            Self::Number => "number",
            Self::Select => "select",
            Self::MultiSelect => "multi_select",
            Self::Date => "date",
            Self::Files => "files",
            Self::Checkbox => "checkbox",
            Self::Url => "url",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Objects => "objects",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Property {
    pub id: String,
    pub key: String,
    pub name: String,
    pub format: PropertyFormat,
    #[serde(default)]
    pub object: String,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}

pub type PropertyListResponse = DataResponse<Property>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyResponse {
    pub property: Property,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PropertyLink {
    pub key: String,
    pub name: String,
    pub format: PropertyFormat,
}

#[derive(Debug, Serialize)]
pub struct CreatePropertyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
    pub format: PropertyFormat,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<CreateTagRequest>,
}

#[derive(Debug, Serialize)]
pub struct UpdatePropertyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Tabled)]
pub struct Tag {
    pub id: String,
    pub key: String,
    pub name: String,
    pub color: IconColor,
    #[serde(default)]
    pub object: String,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}

pub type TagListResponse = DataResponse<Tag>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub tag: Tag,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateTagRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    pub name: String,
    pub color: IconColor,
}

#[derive(Debug, Serialize)]
pub struct UpdateTagRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<IconColor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyLinkValue {
    Text(TextPropertyLinkValue),
    Number(NumberPropertyLinkValue),
    Select(SelectPropertyLinkValue),
    MultiSelect(MultiSelectPropertyLinkValue),
    Date(DatePropertyLinkValue),
    Files(FilesPropertyLinkValue),
    Checkbox(CheckboxPropertyLinkValue),
    Url(UrlPropertyLinkValue),
    Email(EmailPropertyLinkValue),
    Phone(PhonePropertyLinkValue),
    Objects(ObjectsPropertyLinkValue),
}

impl PropertyLinkValue {
    pub fn multi_select(key: impl Into<String>, values: Vec<String>) -> Self {
        Self::MultiSelect(MultiSelectPropertyLinkValue {
            key: key.into(),
            multi_select: values,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TextPropertyLinkValue {
    pub key: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NumberPropertyLinkValue {
    pub key: String,
    pub number: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectPropertyLinkValue {
    pub key: String,
    pub select: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MultiSelectPropertyLinkValue {
    pub key: String,
    pub multi_select: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatePropertyLinkValue {
    pub key: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilesPropertyLinkValue {
    pub key: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckboxPropertyLinkValue {
    pub key: String,
    pub checkbox: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UrlPropertyLinkValue {
    pub key: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmailPropertyLinkValue {
    pub key: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhonePropertyLinkValue {
    pub key: String,
    pub phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObjectsPropertyLinkValue {
    pub key: String,
    pub objects: Vec<String>,
}
