use crate::types::casbin_types::*;
use crate::types::common::AppState;
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use axum::{Json, extract::State};

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
    let result = state
        .casbin
        .add_policy(&req.subject, &req.object, &req.action)
        .await?;
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
    let result = state
        .casbin
        .remove_policy(&req.subject, &req.object, &req.action)
        .await?;
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
    let result = state
        .casbin
        .delete_role_for_user(&req.user, &req.role)
        .await?;
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
    let result = state
        .casbin
        .enforce(&user_str, &req.resource, &req.action)
        .await?;
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
    Ok(ApiResponse::success(
        "Policies reloaded successfully".to_string(),
    ))
}
