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

/// Raw list filter shape returned by Anytype views.
pub type RawListFilter = Value;
/// Raw list sort shape returned by Anytype views.
pub type RawListSort = Value;

#[derive(Debug, Serialize)]
pub struct AddToListRequest<'a> {
    pub objects: &'a [String],
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct ListView {
    pub id: String,
    pub name: String,
    pub layout: String,
    #[serde(default, deserialize_with = "null_or_default")]
    #[tabled(skip)]
    pub filters: Vec<RawListFilter>,
    #[serde(default, deserialize_with = "null_or_default")]
    #[tabled(skip)]
    pub sorts: Vec<RawListSort>,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}
