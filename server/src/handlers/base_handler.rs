// server/src/handlers/crud_handlers.rs
use axum::{extract::{Path, Query, State}, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, 
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set, PrimaryKeyTrait
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::Utc;

use crate::types::{common::*, response::ApiResponse};

/// 通用 CRUD 实体 trait
pub trait CrudEntity: EntityTrait 
where
    <Self::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    Self::Model: Serialize + IntoActiveModel<Self::ActiveModel>,
{
    type CreatePayload: for<'de> Deserialize<'de>;
    type UpdatePayload: for<'de> Deserialize<'de> + HasId;
    type ListPayload: for<'de> Deserialize<'de> + Default;
    
    /// 将创建请求转换为 ActiveModel
    fn create_payload_to_active_model(payload: Self::CreatePayload) -> Self::ActiveModel;
    
    /// 将更新请求应用到 ActiveModel
    fn apply_update_payload(active_model: &mut Self::ActiveModel, payload: Self::UpdatePayload);
    
    /// 应用列表查询过滤条件
    fn apply_list_filters(
        query: sea_orm::Select<Self>,
        payload: &Self::ListPayload,
    ) -> sea_orm::Select<Self> {
        query
    }
    
    /// 创建前的验证和处理
    fn before_create(payload: &Self::CreatePayload) -> Result<(), AppError> {
        Ok(())
    }
    
    /// 更新前的验证和处理
    fn before_update(id: i32, payload: &Self::UpdatePayload) -> Result<(), AppError> {
        Ok(())
    }
    
    /// 删除前的验证和处理
    fn before_delete(id: i32) -> Result<(), AppError> {
        Ok(())
    }
}

/// 通用 CRUD Controller
pub struct BaseController<E: CrudEntity> {
    _phantom: std::marker::PhantomData<E>,
}

impl<E: CrudEntity> BaseController<E> 
where
    <E::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    E::Model: Serialize + IntoActiveModel<E::ActiveModel> + Send + Sync,
    E::ActiveModel: ActiveModelTrait<Entity = E> + Send + Sync + HasDeletedAt,
{
    /// 创建实体
    pub async fn create(
        State(db): State<DatabaseConnection>,
        Json(payload): Json<E::CreatePayload>,
    ) -> Result<ApiResponse<E::Model>, AppError> {
        E::before_create(&payload)?;
        
        let active_model = E::create_payload_to_active_model(payload);
        let entity = active_model.insert(&db).await?;
        
        Ok(ApiResponse::success(entity))
    }

    /// 更新实体
    pub async fn update(
        State(db): State<DatabaseConnection>,
        Json(req): Json<E::UpdatePayload>,
    ) -> Result<ApiResponse<E::Model>, AppError> {
        let id = req.get_id();
        E::before_update(id, &req)?;
        
        let entity = E::find_by_id(id).one(&db).await?;
        let entity = entity.ok_or(AppError::DataNotFound)?;
        
        let mut active_model = entity.into_active_model();
        E::apply_update_payload(&mut active_model, req);
        
        let updated_entity = active_model.update(&db).await?;
        Ok(ApiResponse::success(updated_entity))
    }

    /// 软删除实体
    pub async fn delete(
        State(db): State<DatabaseConnection>,
        Path(id): Path<i32>,
    ) -> Result<ApiResponse<serde_json::Value>, AppError> {
        E::before_delete(id)?;
        
        let entity = E::find_by_id(id).one(&db).await?;
        let entity = entity.ok_or(AppError::DataNotFound)?;
        
        let mut active_model = entity.into_active_model();
        active_model.set_deleted_at(Some(Utc::now().naive_utc()));
        active_model.update(&db).await?;
        
        Ok(ApiResponse::success(json!({ "message": "success" })))
    }

    /// 获取实体列表（分页）
    pub async fn get_list(
        State(db): State<DatabaseConnection>,
        Query(params): Query<ListParamsReq>,
        Json(payload): Json<E::ListPayload>,
    ) -> Result<ApiResponse<PagingResponse<E::Model>>, AppError> {
        let page = params.page;
        let page_size = params.page_size;
        
        let mut query = E::find();
        query = E::apply_list_filters(query, &payload);
        
        let paginator = query.paginate(&db, page_size);
        let total = paginator.num_items().await.unwrap_or(0);
        let list = paginator.fetch_page(page - 1).await?;
        
        Ok(ApiResponse::success(PagingResponse {
            list,
            total,
            page,
        }))
    }

    /// 根据 ID 获取实体
    pub async fn get_by_id(
        State(db): State<DatabaseConnection>,
        Path(id): Path<i32>,
    ) -> Result<ApiResponse<E::Model>, AppError> {
        let entity = E::find_by_id(id).one(&db).await?;
        let entity = entity.ok_or(AppError::DataNotFound)?;
        Ok(ApiResponse::success(entity))
    }
}

/// 辅助 trait：用于获取更新请求中的 ID
pub trait HasId {
    fn get_id(&self) -> i32;
}

/// 辅助 trait：用于设置 deleted_at 字段
pub trait HasDeletedAt {
    fn set_deleted_at(&mut self, deleted_at: Option<chrono::NaiveDateTime>);
}

/// 宏：为 ActiveModel 实现 HasDeletedAt
#[macro_export]
macro_rules! impl_has_deleted_at {
    ($active_model:ty) => {
        impl crate::handlers::base_handler::HasDeletedAt for $active_model {
            fn set_deleted_at(&mut self, deleted_at: Option<chrono::NaiveDateTime>) {
                use sea_orm::Set;
                self.deleted_at = Set(deleted_at);
            }
        }
    };
}

/// 宏：为更新请求实现 HasId
#[macro_export]
macro_rules! impl_has_id {
    ($update_type:ty) => {
        impl crate::handlers::base_handler::HasId for $update_type {
            fn get_id(&self) -> i32 {
                self.id
            }
        }
    };
}

/// 宏：为实体生成带 OpenAPI 文档的 CRUD handlers
#[macro_export]
macro_rules! impl_crud_handlers_with_docs {
    (
        $entity:ty,
        $controller:ty,
        $base_path:literal,
        $entity_name:literal,
        $create_payload:ty,
        $update_payload:ty,
        $list_payload:ty
    ) => {
        #[utoipa::path(
            post,
            path = concat!($base_path),
            request_body = $create_payload,
            responses(
                (status = 200, description = concat!($entity_name, " created successfully"), body = ApiResponse<entity::apps::Model>),
            ),
            security(("api_key" = []))
        )]
        pub async fn create(
            state: axum::extract::State<sea_orm::DatabaseConnection>,
            json: axum::Json<$create_payload>,
        ) -> Result<crate::types::response::ApiResponse<<$entity as sea_orm::EntityTrait>::Model>, crate::types::common::AppError> {
            <$controller>::create(state, json).await
        }

        #[utoipa::path(
            put,
            path = concat!($base_path, "/{id}"),
            request_body = $update_payload,
            responses(
                (status = 200, description = concat!($entity_name, " updated successfully"), body = ApiResponse<entity::apps::Model>),
            ),
            security(("api_key" = []))
        )]
        pub async fn update(
            state: axum::extract::State<sea_orm::DatabaseConnection>,
            json: axum::Json<$update_payload>,
        ) -> Result<crate::types::response::ApiResponse<<$entity as sea_orm::EntityTrait>::Model>, crate::types::common::AppError> {
            <$controller>::update(state, json).await
        }

        #[utoipa::path(
            delete,
            path = concat!($base_path, "/{id}"),
            responses(
                (status = 200, description = concat!($entity_name, " deleted successfully"), body = ApiResponse<serde_json::Value>),
            ),
            security(("api_key" = []))
        )]
        pub async fn delete(
            state: axum::extract::State<sea_orm::DatabaseConnection>,
            path: axum::extract::Path<i32>,
        ) -> Result<crate::types::response::ApiResponse<serde_json::Value>, crate::types::common::AppError> {
            <$controller>::delete(state, path).await
        }

        #[utoipa::path(
            get,
            path = concat!($base_path),
            responses(
                (status = 200, description = concat!($entity_name, " list retrieved successfully"), body = ApiResponse<PagingResponse<entity::apps::Model>>),
            ),
            security(("api_key" = []))
        )]
        pub async fn get_list(
            state: axum::extract::State<sea_orm::DatabaseConnection>,
            query: axum::extract::Query<crate::types::common::ListParamsReq>,
            json: axum::Json<$list_payload>,
        ) -> Result<crate::types::response::ApiResponse<crate::types::common::PagingResponse<<$entity as sea_orm::EntityTrait>::Model>>, crate::types::common::AppError> {
            <$controller>::get_list(state, query, json).await
        }

        #[utoipa::path(
            get,
            path = concat!($base_path, "/{id}"),
            responses(
                (status = 200, description = concat!($entity_name, " retrieved successfully"), body = ApiResponse<entity::apps::Model>),
            ),
            security(("api_key" = []))
        )]
        pub async fn get_by_id(
            state: axum::extract::State<sea_orm::DatabaseConnection>,
            path: axum::extract::Path<i32>,
        ) -> Result<crate::types::response::ApiResponse<<$entity as sea_orm::EntityTrait>::Model>, crate::types::common::AppError> {
            <$controller>::get_by_id(state, path).await
        }
    };
}