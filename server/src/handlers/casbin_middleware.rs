use crate::types::common::{AppState, Claims};
use crate::types::error::AppError;
use salvo::prelude::*;

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

#[handler]
pub async fn casbin_auth(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<(), StatusCode> {
    let state=match depot.obtain::<AppState>() {
        Ok(s) => s,
        Err(_) => {
            // res.status_code = Some(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let claims=match depot.obtain::<Claims>() {
        Ok(c) => c,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let user_id = claims.sub;
    let role = &claims.role;
    let has_permission = match check_path_permission(state, user_id, role, &path, method.to_string().as_str()).await {
        Ok(h) => h,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    if !has_permission {
        // res.status_code = Some(StatusCode::FORBIDDEN);
        return Err(StatusCode::FORBIDDEN);
    }
    tracing::debug!("Permission granted for user {} (role: {}) to {} {}", user_id, role, method, path);
    // ctrl.call_next(req, depot, res).await;
    Ok(())
}
