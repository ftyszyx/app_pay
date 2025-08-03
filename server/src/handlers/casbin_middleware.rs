use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode, Method},
    middleware::Next,
    response::Response,
    Extension,
};
use crate::types::common::{AppState, Claims};
use crate::types::error::AppError;

/// Casbin权限检查中间件
pub async fn casbin_auth(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = req.method().clone();
    let path = req.uri().path();
    
    // 转换HTTP方法为权限动作
    let action = match method {
        Method::GET => "read",
        Method::POST => "create", 
        Method::PUT => "update",
        Method::DELETE => "delete",
        _ => return Err(StatusCode::METHOD_NOT_ALLOWED),
    };

    // 构建用户标识 - 可以使用用户ID或角色
    let user_id = claims.sub.to_string();
    let role = &claims.role;

    // 检查权限 - 先检查用户权限，再检查角色权限
    let has_user_permission = state.casbin.enforce(&user_id, path, action).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let has_role_permission = state.casbin.enforce(role, path, action).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !has_user_permission && !has_role_permission {
        tracing::warn!(
            "Access denied for user {} (role: {}) to {} {}",
            user_id, role, method, path
        );
        return Err(StatusCode::FORBIDDEN);
    }

    tracing::debug!(
        "Permission granted for user {} (role: {}) to {} {}",
        user_id, role, method, path
    );

    Ok(next.run(req).await)
}

/// 权限检查函数（用于手动检查）
pub async fn check_permission(
    state: &AppState,
    user_id: i32,
    role: &str,
    resource: &str,
    action: &str,
) -> Result<bool, AppError> {
    let user_str = user_id.to_string();
    
    // 检查用户权限
    let user_permission = state.casbin.enforce(&user_str, resource, action).await?;
    if user_permission {
        return Ok(true);
    }
    
    // 检查角色权限
    let role_permission = state.casbin.enforce(role, resource, action).await?;
    Ok(role_permission)
}

/// 路径匹配权限检查（支持通配符）
pub async fn check_path_permission(
    state: &AppState,
    user_id: i32,
    role: &str,
    path: &str,
    method: &str,
) -> Result<bool, AppError> {
    let action = match method.to_uppercase().as_str() {
        "GET" => "read",
        "POST" => "create",
        "PUT" => "update", 
        "DELETE" => "delete",
        _ => return Ok(false),
    };
    
    check_permission(state, user_id, role, path, action).await
}