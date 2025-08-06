use crate::types::error::AppError;
use crate::utils::casbin_adapter::CasbinAdapter;
use casbin::{CoreApi, Enforcer, MgmtApi};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type CasbinEnforcer = Arc<RwLock<Enforcer>>;

#[derive(Clone)]
pub struct CasbinService {
    pub enforcer: CasbinEnforcer,
}

impl CasbinService {
    pub async fn new(db_pool: &DatabaseConnection) -> Result<Self, AppError> {
        let adapter = CasbinAdapter::new(db_pool.clone());
        let enforcer = Enforcer::new("casbin_model.conf", adapter)
            .await
            .map_err(|e| AppError::InternalError {
                message: format!("Failed to create Casbin enforcer: {}", e),
            })?;

        let enforcer = Arc::new(RwLock::new(enforcer));

        Ok(Self { enforcer })
    }

    // 检查权限
    pub async fn enforce(&self, sub: &str, obj: &str, act: &str) -> Result<bool, AppError> {
        let e = self.enforcer.read().await;
        e.enforce((sub, obj, act))
            .map_err(|e| AppError::InternalError {
                message: format!("Permission check failed: {}", e),
            })
    }

    // 添加策略
    pub async fn add_policy(&self, sub: &str, obj: &str, act: &str) -> Result<bool, AppError> {
        let mut e = self.enforcer.write().await;
        e.add_policy(vec![sub.to_string(), obj.to_string(), act.to_string()])
            .await
            .map_err(|e| AppError::InternalError {
                message: format!("Failed to add policy: {}", e),
            })
    }

    // 删除策略
    pub async fn remove_policy(&self, sub: &str, obj: &str, act: &str) -> Result<bool, AppError> {
        let mut e = self.enforcer.write().await;
        e.remove_policy(vec![sub.to_string(), obj.to_string(), act.to_string()])
            .await
            .map_err(|e| AppError::InternalError {
                message: format!("Failed to remove policy: {}", e),
            })
    }

    // 添加角色继承
    pub async fn add_role_for_user(&self, user: &str, role: &str) -> Result<bool, AppError> {
        let mut e = self.enforcer.write().await;
        e.add_grouping_policy(vec![user.to_string(), role.to_string()])
            .await
            .map_err(|e| AppError::InternalError {
                message: format!("Failed to add role: {}", e),
            })
    }

    // 删除用户角色
    pub async fn delete_role_for_user(&self, user: &str, role: &str) -> Result<bool, AppError> {
        let mut e = self.enforcer.write().await;
        e.remove_grouping_policy(vec![user.to_string(), role.to_string()])
            .await
            .map_err(|e| AppError::InternalError {
                message: format!("Failed to delete role: {}", e),
            })
    }

    // 获取所有策略
    pub async fn get_policy(&self) -> Result<Vec<Vec<String>>, AppError> {
        let e = self.enforcer.read().await;
        Ok(e.get_policy())
    }

    // 获取所有角色继承关系
    pub async fn get_grouping_policy(&self) -> Result<Vec<Vec<String>>, AppError> {
        let e = self.enforcer.read().await;
        Ok(e.get_grouping_policy())
    }

    // 刷新策略（从数据库重新加载）
    pub async fn load_policy(&self) -> Result<(), AppError> {
        let mut e = self.enforcer.write().await;
        e.load_policy().await.map_err(|e| AppError::InternalError {
            message: format!("Failed to load policy: {}", e),
        })
    }

    // 保存策略到数据库
    pub async fn save_policy(&self) -> Result<(), AppError> {
        let mut e = self.enforcer.write().await;
        e.save_policy().await.map_err(|e| AppError::InternalError {
            message: format!("Failed to save policy: {}", e),
        })
    }
}
