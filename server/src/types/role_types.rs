use crate::types::common::ListParamsReq;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RoleCreatePayload {
    pub name: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct RoleUpdatePayload {
    pub name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct RoleListResponse {
    pub list: Vec<entity::roles::Model>,
    pub total: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListRolesParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    pub id: Option<i32>,
    pub name: Option<String>,
}
