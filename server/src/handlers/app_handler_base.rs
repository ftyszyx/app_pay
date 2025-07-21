use entity::apps;
use sea_orm::{Set, ColumnTrait, QueryFilter};
use crate::types::app_types::*;
use crate::handlers::base_handler::{CrudEntity, BaseController, HasId, HasDeletedAt};
use crate::types::common::AppError;

// 为 apps::ActiveModel 实现 HasDeletedAt trait
crate::impl_has_deleted_at!(apps::ActiveModel);

// 为 UpdateAppReq 实现 HasId trait
crate::impl_has_id!(UpdateAppReq);

/// App 实体的 CRUD 实现
impl CrudEntity for apps::Entity {
    type CreatePayload = AddAppReq;
    type UpdatePayload = UpdateAppReq;
    type ListPayload = ListAppsParams;

    /// 将创建请求转换为 ActiveModel
    fn create_payload_to_active_model(payload: Self::CreatePayload) -> Self::ActiveModel {
        apps::ActiveModel {
            name: Set(payload.name),
            app_id: Set(payload.app_id),
            app_vername: Set(payload.app_vername),
            app_vercode: Set(payload.app_vercode),
            app_download_url: Set(payload.app_download_url),
            app_res_url: Set(payload.app_res_url),
            app_update_info: Set(payload.app_update_info),
            sort_order: Set(payload.sort_order),
            status: Set(payload.status),
            ..Default::default()
        }
    }

    /// 将更新请求应用到 ActiveModel
    fn apply_update_payload(active_model: &mut Self::ActiveModel, payload: Self::UpdatePayload) {
        if let Some(name) = payload.name {
            active_model.name = Set(name);
        }
        if let Some(app_id) = payload.app_id {
            active_model.app_id = Set(app_id);
        }
        if let Some(app_vername) = payload.app_vername {
            active_model.app_vername = Set(app_vername);
        }
        if let Some(app_vercode) = payload.app_vercode {
            active_model.app_vercode = Set(app_vercode);
        }
        if let Some(app_download_url) = payload.app_download_url {
            active_model.app_download_url = Set(app_download_url);
        }
        if let Some(app_res_url) = payload.app_res_url {
            active_model.app_res_url = Set(app_res_url);
        }
        if let Some(app_update_info) = payload.app_update_info {
            active_model.app_update_info = Set(app_update_info);
        }
        if let Some(sort_order) = payload.sort_order {
            active_model.sort_order = Set(sort_order);
        }
        if let Some(status) = payload.status {
            active_model.status = Set(status);
        }
    }

    /// 应用列表查询过滤条件
    fn apply_list_filters(
        mut query: sea_orm::Select<Self>,
        payload: &Self::ListPayload,
    ) -> sea_orm::Select<Self> {
        // 过滤软删除的记录
        query = query.filter(apps::Column::DeletedAt.is_null());
        
        // 按创建时间倒序排列
        query = query.order_by_desc(apps::Column::CreatedAt);
        
        // 根据名称过滤
        if let Some(name) = &payload.name {
            if !name.is_empty() {
                query = query.filter(apps::Column::Name.contains(name));
            }
        }
        
        query
    }

    /// 创建前的验证
    fn before_create(payload: &Self::CreatePayload) -> Result<(), AppError> {
        // 可以添加业务逻辑验证，比如检查 app_id 是否重复等
        if payload.name.trim().is_empty() {
            return Err(AppError::DataNotFound); // 这里应该用更合适的错误类型
        }
        Ok(())
    }

    /// 更新前的验证
    fn before_update(id: i32, payload: &Self::UpdatePayload) -> Result<(), AppError> {
        // 可以添加业务逻辑验证
        if let Some(name) = &payload.name {
            if name.trim().is_empty() {
                return Err(AppError::DataNotFound); // 这里应该用更合适的错误类型
            }
        }
        Ok(())
    }
}

/// App CRUD Controller 类型别名
pub type AppController = BaseController<apps::Entity>;

// 使用宏生成带 OpenAPI 文档的 CRUD handlers
crate::impl_crud_handlers_with_docs!(
    apps::Entity,
    AppController,
    "/api/admin/apps",
    "App",
    AddAppReq,
    UpdateAppReq,
    ListAppsParams
); 