use crate::types::common::AppState;
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use axum::{Json, extract::State};
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
    pub user: String,  // 用户ID
    pub role: String,  // 角色名
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

#[derive(Serialize, ToSchema, Debug)]
pub struct PermissionCheckReq {
    pub user_id: i32,
    pub resource: String,
    pub action: String,
}

// 添加权限策略
#[utoipa::path(
    post,
    path = "/api/admin/permissions/policies",
    security(("api_key" = [])),
    request_body = AddPolicyReq,
    responses((status = 200, description = "Success", body = ApiResponse<bool>))
)]
pub async fn add_policy(
    State(state): State<AppState>,
    Json(req): Json<AddPolicyReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let result = state.casbin.add_policy(&req.subject, &req.object, &req.action).await?;
    Ok(ApiResponse::success(result))
}

// 删除权限策略
#[utoipa::path(
    delete,
    path = "/api/admin/permissions/policies",
    security(("api_key" = [])),
    request_body = RemovePolicyReq,
    responses((status = 200, description = "Success", body = ApiResponse<bool>))
)]
pub async fn remove_policy(
    State(state): State<AppState>,
    Json(req): Json<RemovePolicyReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let result = state.casbin.remove_policy(&req.subject, &req.object, &req.action).await?;
    Ok(ApiResponse::success(result))
}

// 为用户添加角色
#[utoipa::path(
    post,
    path = "/api/admin/permissions/roles",
    security(("api_key" = [])),
    request_body = AddRoleReq,
    responses((status = 200, description = "Success", body = ApiResponse<bool>))
)]
pub async fn add_role_for_user(
    State(state): State<AppState>,
    Json(req): Json<AddRoleReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let result = state.casbin.add_role_for_user(&req.user, &req.role).await?;
    Ok(ApiResponse::success(result))
}

// 删除用户角色
#[utoipa::path(
    delete,
    path = "/api/admin/permissions/roles",
    security(("api_key" = [])),
    request_body = RemoveRoleReq,
    responses((status = 200, description = "Success", body = ApiResponse<bool>))
)]
pub async fn remove_role_for_user(
    State(state): State<AppState>,
    Json(req): Json<RemoveRoleReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let result = state.casbin.delete_role_for_user(&req.user, &req.role).await?;
    Ok(ApiResponse::success(result))
}

// 获取所有权限策略
#[utoipa::path(
    get,
    path = "/api/admin/permissions/policies",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = ApiResponse<Vec<PolicyInfo>>))
)]
pub async fn get_policies(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<PolicyInfo>>, AppError> {
    let policies = state.casbin.get_policy().await?;
    let policy_infos: Vec<PolicyInfo> = policies
        .into_iter()
        .filter(|p| p.len() >= 3)
        .map(|p| PolicyInfo {
            subject: p[0].clone(),
            object: p[1].clone(),
            action: p[2].clone(),
        })
        .collect();
    Ok(ApiResponse::success(policy_infos))
}

// 获取所有角色关系
#[utoipa::path(
    get,
    path = "/api/admin/permissions/roles",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = ApiResponse<Vec<RoleInfo>>))
)]
pub async fn get_roles(
    State(state): State<AppState>,
) -> Result<ApiResponse<Vec<RoleInfo>>, AppError> {
    let roles = state.casbin.get_grouping_policy().await?;
    let role_infos: Vec<RoleInfo> = roles
        .into_iter()
        .filter(|r| r.len() >= 2)
        .map(|r| RoleInfo {
            user: r[0].clone(),
            role: r[1].clone(),
        })
        .collect();
    Ok(ApiResponse::success(role_infos))
}

// 获取用户的所有角色
#[utoipa::path(
    get,
    path = "/api/admin/permissions/users/{user_id}/roles",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = ApiResponse<Vec<String>>))
)]
pub async fn get_user_roles(
    State(state): State<AppState>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<ApiResponse<Vec<String>>, AppError> {
    let roles = state.casbin.get_roles_for_user(&user_id).await?;
    Ok(ApiResponse::success(roles))
}

// 获取角色的所有用户
#[utoipa::path(
    get,
    path = "/api/admin/permissions/roles/{role}/users",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = ApiResponse<Vec<String>>))
)]
pub async fn get_role_users(
    State(state): State<AppState>,
    axum::extract::Path(role): axum::extract::Path<String>,
) -> Result<ApiResponse<Vec<String>>, AppError> {
    let users = state.casbin.get_users_for_role(&role).await?;
    Ok(ApiResponse::success(users))
}

// 检查权限
#[utoipa::path(
    post,
    path = "/api/admin/permissions/check",
    security(("api_key" = [])),
    request_body = PermissionCheckReq,
    responses((status = 200, description = "Success", body = ApiResponse<bool>))
)]
pub async fn check_permission(
    State(state): State<AppState>,
    Json(req): Json<PermissionCheckReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let user_str = req.user_id.to_string();
    let result = state.casbin.enforce(&user_str, &req.resource, &req.action).await?;
    Ok(ApiResponse::success(result))
}

// 重新加载策略
#[utoipa::path(
    post,
    path = "/api/admin/permissions/reload",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = ApiResponse<String>))
)]
pub async fn reload_policies(
    State(state): State<AppState>,
) -> Result<ApiResponse<String>, AppError> {
    state.casbin.load_policy().await?;
    Ok(ApiResponse::success("Policies reloaded successfully".to_string()))
}