use crate::types::common::AppState;
use crate::types::error::AppError;
use tracing::{info, error};

/// 初始化Casbin权限策略
pub async fn init_casbin_policies(state: &AppState) -> Result<(), AppError> {
    info!("Initializing Casbin policies...");
    
    // 定义默认角色
    let roles = vec![
        "admin",
        "user",
        "guest",
    ];

    // 定义管理员权限 - 管理员可以访问所有资源
    let admin_policies = vec![
        ("admin", "/api/admin/*", "create"),
        ("admin", "/api/admin/*", "read"),
        ("admin", "/api/admin/*", "update"),
        ("admin", "/api/admin/*", "delete"),
    ];

    // 定义用户权限 - 普通用户只能访问部分资源
    let user_policies = vec![
        ("user", "/api/admin/me", "read"),
        ("user", "/api/admin/products/list", "read"),
        ("user", "/api/admin/products/*", "read"),
        ("user", "/api/admin/orders", "create"),
        ("user", "/api/admin/orders/list", "read"),
        ("user", "/api/admin/orders/*", "read"),
    ];

    // 定义访客权限 - 访客只能查看公开信息
    let guest_policies = vec![
        ("guest", "/api/admin/products/list", "read"),
        ("guest", "/api/admin/products/*", "read"),
    ];

    // 添加管理员权限
    for (role, resource, action) in admin_policies {
        match state.casbin.add_policy(role, resource, action).await {
            Ok(_) => info!("Added policy: {} -> {} [{}]", role, resource, action),
            Err(e) => error!("Failed to add policy {}->{}: {}", role, resource, e),
        }
    }

    // 添加用户权限
    for (role, resource, action) in user_policies {
        match state.casbin.add_policy(role, resource, action).await {
            Ok(_) => info!("Added policy: {} -> {} [{}]", role, resource, action),
            Err(e) => error!("Failed to add policy {}->{}: {}", role, resource, e),
        }
    }

    // 添加访客权限
    for (role, resource, action) in guest_policies {
        match state.casbin.add_policy(role, resource, action).await {
            Ok(_) => info!("Added policy: {} -> {} [{}]", role, resource, action),
            Err(e) => error!("Failed to add policy {}->{}: {}", role, resource, e),
        }
    }

    // 可以在这里为特定用户分配角色
    // 例如：为用户ID为1的用户分配admin角色
    match state.casbin.add_role_for_user("1", "admin").await {
        Ok(_) => info!("Assigned admin role to user 1"),
        Err(e) => error!("Failed to assign admin role to user 1: {}", e),
    }

    info!("Casbin policies initialization completed");
    Ok(())
}

/// 为新用户设置默认权限
pub async fn setup_default_user_permission(
    state: &AppState, 
    user_id: i32, 
    role: &str
) -> Result<(), AppError> {
    let user_str = user_id.to_string();
    
    // 为用户分配角色
    state.casbin.add_role_for_user(&user_str, role).await?;
    
    info!("Assigned role '{}' to user {}", role, user_id);
    Ok(())
}

/// 检查并创建基础权限数据
pub async fn ensure_basic_permissions(state: &AppState) -> Result<(), AppError> {
    info!("Ensuring basic permissions exist...");
    
    // 确保超级管理员权限存在
    let super_admin_policies = vec![
        ("super_admin", "/*", "create"),
        ("super_admin", "/*", "read"), 
        ("super_admin", "/*", "update"),
        ("super_admin", "/*", "delete"),
    ];

    for (role, resource, action) in super_admin_policies {
        state.casbin.add_policy(role, resource, action).await?;
    }

    info!("Basic permissions ensured");
    Ok(())
}