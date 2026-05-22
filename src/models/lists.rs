use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::Tabled;

use super::{DataResponse, Object};

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
