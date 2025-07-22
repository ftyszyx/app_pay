use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct RoleCreatePayload {
    pub name: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct RoleUpdatePayload {
    pub name: Option<String>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct RoleListResponse {
    pub list: Vec<entity::roles::Model>,
    pub total: u64,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct ListRolesParams {
    pub name: Option<String>,
}
