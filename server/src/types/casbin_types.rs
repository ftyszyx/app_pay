use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct AddPolicyReq {
    pub subject: String, // 用户ID或角色名
    pub object: String,  // 资源路径
    pub action: String,  // 操作类型：read, create, update, delete
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct RemovePolicyReq {
    pub subject: String,
    pub object: String,
    pub action: String,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct AddRoleReq {
    pub user: String, // 用户ID
    pub role: String, // 角色名
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct RemoveRoleReq {
    pub user: String,
    pub role: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct PolicyInfo {
    pub subject: String,
    pub object: String,
    pub action: String,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct RoleInfo {
    pub user: String,
    pub role: String,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct PermissionCheckReq {
    pub user_id: i32,
    pub resource: String,
    pub action: String,
}
