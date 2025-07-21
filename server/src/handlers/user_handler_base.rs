use entity::users;
use sea_orm::{Set, ColumnTrait, QueryFilter};
use uuid::Uuid;
use crate::types::user_types::*;
use crate::handlers::base_handler::{CrudEntity, BaseController, HasId, HasDeletedAt};
use crate::types::common::AppError;
use crate::constants;

// 为 users::ActiveModel 实现 HasDeletedAt trait
crate::impl_has_deleted_at!(users::ActiveModel);

// 为 UserUpdatePayload 实现 HasId trait
impl crate::handlers::base_handler::HasId for UserUpdatePayload {
    fn get_id(&self) -> i32 {
        self.id.unwrap_or(0) // 假设 UserUpdatePayload 有 id 字段
    }
}

/// User 实体的 CRUD 实现
impl CrudEntity for users::Entity {
    type CreatePayload = UserCreatePayload;
    type UpdatePayload = UserUpdatePayload;
    type ListPayload = ListUsersParams;

    /// 将创建请求转换为 ActiveModel
    fn create_payload_to_active_model(payload: Self::CreatePayload) -> Self::ActiveModel {
        let user_id = Uuid::new_v4().to_string();
        users::ActiveModel {
            user_id: Set(user_id),
            username: Set(payload.username),
            // 安全地哈希密码
            password: Set(bcrypt::hash(payload.password, 10).unwrap()),
            role_id: Set(payload.role_id.unwrap_or(constants::DEFAULT_ROLE_ID)),
            ..Default::default()
        }
    }

    /// 将更新请求应用到 ActiveModel
    fn apply_update_payload(active_model: &mut Self::ActiveModel, payload: Self::UpdatePayload) {
        if let Some(username) = payload.username {
            active_model.username = Set(username);
        }
        if let Some(password) = payload.password {
            // 安全地哈希新密码
            active_model.password = Set(bcrypt::hash(password, 10).unwrap());
        }
        if let Some(role_id) = payload.role_id {
            active_model.role_id = Set(role_id);
        }
        if let Some(balance) = payload.balance {
            active_model.balance = Set(balance);
        }
    }

    /// 应用列表查询过滤条件
    fn apply_list_filters(
        mut query: sea_orm::Select<Self>,
        payload: &Self::ListPayload,
    ) -> sea_orm::Select<Self> {
        // 按 ID 倒序排列
        query = query.order_by_desc(users::Column::Id);
        
        // 根据用户名过滤
        if let Some(username) = &payload.username {
            if !username.is_empty() {
                query = query.filter(users::Column::Username.contains(username));
            }
        }
        
        query
    }

    /// 创建前的验证
    fn before_create(payload: &Self::CreatePayload) -> Result<(), AppError> {
        // 验证用户名不为空
        if payload.username.trim().is_empty() {
            return Err(AppError::DataNotFound); // 应该用更合适的错误类型
        }
        
        // 验证密码强度（示例）
        if payload.password.len() < 6 {
            return Err(AppError::DataNotFound); // 应该用更合适的错误类型
        }
        
        Ok(())
    }

    /// 更新前的验证
    fn before_update(id: i32, payload: &Self::UpdatePayload) -> Result<(), AppError> {
        // 验证用户名不为空
        if let Some(username) = &payload.username {
            if username.trim().is_empty() {
                return Err(AppError::DataNotFound); // 应该用更合适的错误类型
            }
        }
        
        // 验证密码强度
        if let Some(password) = &payload.password {
            if password.len() < 6 {
                return Err(AppError::DataNotFound); // 应该用更合适的错误类型
            }
        }
        
        Ok(())
    }
}

/// User CRUD Controller 类型别名
pub type UserController = BaseController<users::Entity>;

// 导出具体的 handler 函数，这些可以直接在路由中使用
pub use UserController::create as create_user_base;
pub use UserController::update as update_user_base;
pub use UserController::delete as delete_user_base;
pub use UserController::get_list as get_users_list_base;
pub use UserController::get_by_id as get_user_by_id_base; 