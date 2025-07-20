use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RoleCreatePayload {
    pub name: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct RoleUpdatePayload {
    pub name: Option<String>,
    pub remark: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct RoleListResponse {
    pub list: Vec<entity::roles::Model>,
    pub total: u64,
} 