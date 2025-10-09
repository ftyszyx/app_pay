use crate::types::common::{AppState, Claims};
use crate::types::error::AppError;
use salvo::http::Method;
use salvo::prelude::*;

#[handler]
pub async fn casbin_auth(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let state = match depot.obtain::<AppState>() {
        Ok(s) => s,
        Err(_) => {
            res.status_code = Some(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };
    let claims = match depot.obtain::<Claims>() {
        Ok(c) => c,
        Err(_) => {
            res.status_code = Some(StatusCode::UNAUTHORIZED);
            return;
        }
    };

    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let action = match method {
        Method::GET => "read",
        Method::POST => "create",
        Method::PUT => "update",
        Method::DELETE => "delete",
        _ => {
            res.status_code = Some(StatusCode::METHOD_NOT_ALLOWED);
            return;
        }
    };

    let user_id = claims.sub.to_string();
    let role = &claims.role;

    let has_user = match state.casbin.enforce(&user_id, &path, action).await {
        Ok(v) => v,
        Err(_) => {
            res.status_code = Some(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };
    let has_role = match state.casbin.enforce(role, &path, action).await {
        Ok(v) => v,
        Err(_) => {
            res.status_code = Some(StatusCode::INTERNAL_SERVER_ERROR);
            return;
        }
    };

    if !has_user && !has_role {
        tracing::warn!("Access denied for user {} (role: {}) to {} {}", user_id, role, method, path);
        res.status_code = Some(StatusCode::FORBIDDEN);
        return;
    }

    tracing::debug!("Permission granted for user {} (role: {}) to {} {}", user_id, role, method, path);
    ctrl.call_next(req, depot, res).await;
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
    let user_permission = state.casbin.enforce(&user_str, resource, action).await?;
    if user_permission {
        return Ok(true);
    }
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