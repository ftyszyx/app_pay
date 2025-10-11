use crate::handlers::role_handler;
use crate::{handlers::user_handler, types::casbin_types::*};
use crate::types::common::AppState;
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use salvo::{prelude::*, oapi::extract::JsonBody};

// 添加权限策略
#[handler]
pub async fn add_policy(
    depot: &mut Depot,
    req: JsonBody<AddPolicyReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let req = req.into_inner();
    let result = state
        .casbin
        .add_policy(&req.subject, &req.object, &req.action)
        .await?;
    Ok(ApiResponse::success(result))
}

// 删除权限策略
#[handler]
pub async fn remove_policy(
    depot: &mut Depot,
    req: JsonBody<RemovePolicyReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let req = req.into_inner();
    let result = state
        .casbin
        .remove_policy(&req.subject, &req.object, &req.action)
        .await?;
    Ok(ApiResponse::success(result))
}

// 为用户添加角色
#[handler]
pub async fn add_role_for_user(
    depot: &mut Depot,
    req: JsonBody<AddRoleReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let req = req.into_inner();
    //get userinfo from database
    let userinfo = user_handler::get_by_id_impl(&state, req.user_id).await?;
    //get roleinfo from database
    let roleinfo = role_handler::get_by_id_impl(&state, req.role_id).await?;
    let role_str = roleinfo.name;
    let result = state.casbin.add_role_for_user(&userinfo.id.to_string(), &role_str).await?;
    Ok(ApiResponse::success(result))
}

// 删除用户角色
#[handler]
pub async fn remove_role_for_user(
    depot: &mut Depot,
    req: JsonBody<RemoveRoleReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let req = req.into_inner();
    //get userinfo from database
    let userinfo = user_handler::get_by_id_impl(&state, req.user_id).await?;
    //get roleinfo from database
    let roleinfo = role_handler::get_by_id_impl(&state, req.role_id).await?;
    let role_str = roleinfo.name;
    let result = state
        .casbin
        .delete_role_for_user(&userinfo.id.to_string(), &role_str)
        .await?;
    Ok(ApiResponse::success(result))
}

// 获取所有权限策略
#[handler]
pub async fn get_policies(
    depot: &mut Depot,
) -> Result<ApiResponse<Vec<PolicyInfo>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
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
#[handler]
pub async fn get_roles(
    depot: &mut Depot,
) -> Result<ApiResponse<Vec<RoleInfo>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let roles = state.casbin.get_grouping_policy().await?;
    let roleinfos=roles.into_iter().filter(|r| r.len() >= 2);
    let mut role_infos: Vec<RoleInfo> = Vec::new();
    for r in roleinfos {
        let user_id = r[0].clone();
        //can return error
        let user_id = user_id.parse::<i32>().map_err(|e| AppError::Message(e.to_string()))?;
        let user = user_handler::get_by_id_impl(&state, user_id).await?;
        role_infos.push(RoleInfo {
            user_id: user.id,
            user: user.username,
            role: r[1].clone(),
        });
    }
    Ok(ApiResponse::success(role_infos))
}

// 检查权限
#[handler]
pub async fn check_permission(
    depot: &mut Depot,
    req: JsonBody<PermissionCheckReq>,
) -> Result<ApiResponse<bool>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let req = req.into_inner();
    //get userinfo from database
    let userinfo = user_handler::get_by_id_impl(&state, req.user_id).await?;
    let user_str = userinfo.username;
    let result = state
        .casbin
        .enforce(&user_str, &req.resource, &req.action)
        .await?;
    Ok(ApiResponse::success(result))
}

// 重新加载策略
#[handler]
pub async fn reload_policies(
    depot: &mut Depot,
) -> Result<ApiResponse<String>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    state.casbin.load_policy().await?;
    Ok(ApiResponse::success(
        "Policies reloaded successfully".to_string(),
    ))
}
