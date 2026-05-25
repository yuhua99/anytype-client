use serde::{Deserialize, Serialize};
use tabled::Tabled;

use super::{DataResponse, ExtraFields, Icon};

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
    #[serde(flatten)]
    #[tabled(skip)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberResponse {
    pub member: Member,
}
