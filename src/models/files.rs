use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::ExtraFields;

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct FileUploadResponse {
    pub object_id: String,
    pub name: String,
    pub media: String,
    pub extension: String,
    pub size_in_bytes: i64,
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}
