use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AddPolicyReq {
    pub subject: String, // 用户ID或角色名
    pub object: String,  // 资源路径
    pub action: String,  // 操作类型：read, create, update, delete
}

#[derive(Deserialize, Debug)]
pub struct RemovePolicyReq {
    pub subject: String,
    pub object: String,
    pub action: String,
}

#[derive(Deserialize, Debug)]
pub struct AddRoleReq {
    pub user_id: i32, // 用户ID
    pub role_id: i32, // 角色名
}

#[derive(Deserialize, Debug)]
pub struct RemoveRoleReq {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Serialize, Debug)]
pub struct PolicyInfo {
    pub subject: String,
    pub object: String,
    pub action: String,
}

#[derive(Serialize, Debug)]
pub struct RoleInfo {
    pub user_id: i32,
    pub user: String,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct PermissionCheckReq {
    pub user_id: i32,
    pub resource: String,
    pub action: String,
}
