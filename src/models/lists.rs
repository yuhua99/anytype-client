use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use tabled::Tabled;

use super::{DataResponse, ExtraFields, Object};

fn null_or_default<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Option::<Vec<T>>::deserialize(deserializer)?.unwrap_or_default())
}

pub type ViewListResponse = DataResponse<ListView>;
pub type ObjectListResponse = DataResponse<Object>;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct ListView {
    pub id: String,
    pub name: String,
    pub layout: String,
    #[serde(default, deserialize_with = "null_or_default")]
    #[tabled(skip)]
    pub filters: Vec<Value>,
    #[serde(default, deserialize_with = "null_or_default")]
    #[tabled(skip)]
    pub sorts: Vec<Value>,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}
