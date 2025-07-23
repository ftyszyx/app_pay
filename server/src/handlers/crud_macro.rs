// 统一的CRUD宏 - 完整实现
#[macro_export]
macro_rules! impl_crud_handlers {
    (
        $handler:ident,
        $entity:ty,
        $active_model:ty,
        $model:ty,
        $create_payload:ty,
        $update_payload:ty,
        $search_payload:ty,
        $search_result:ty,
        $model_name:literal,
        $fake_delete:tt
    ) => {
        use crate::types::common::{ ListParamsReq, PagingResponse};
        use crate::types::error::AppError;
        use crate::types::response::ApiResponse;
        use axum::{
            extract::{Path, Query, State},
            Json,
        };
        use chrono::Utc;
        use sea_orm::{
            ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
            QueryFilter, QueryOrder, Set,
        };
        use crate::types::crud::CrudOperations;

        pub struct $handler;

        #[utoipa::path(
            post,
            path = concat!("/api/admin/", $model_name),
            security(("api_key" = [])),
            request_body = $create_payload,
            responses((status = 200, description = "Success", body = $model))
        )]
        pub async fn add(
            State(db): State<sea_orm::DatabaseConnection>,
            Json(req): Json<$create_payload>,
        ) -> Result<ApiResponse<$model>, AppError> {
            $handler::before_add(&req)?;
            let new_entity = $handler::create_model(req);
            let entity = new_entity.insert(&db).await?;
            $handler::after_add(&entity)?;
            Ok(ApiResponse::success(entity))
        }

        #[utoipa::path(
            put,
            path = concat!("/api/admin/", $model_name, "/{id}"),
            security(("api_key" = [])),
            request_body = $update_payload,
            responses((status = 200, description = "Success", body = $model))
        )]
        pub async fn update(
            State(db): State<sea_orm::DatabaseConnection>,
            Path(id): Path<i32>,
            Json(req): Json<$update_payload>,
        ) -> Result<ApiResponse<$model>, AppError> {
            $handler::before_update(id, &req)?;
            let app = <$entity>::find_by_id(id).one(&db).await?;
            let app = app.ok_or_else(|| AppError::not_found(stringify!($model_name).to_string(), Some(id)))?;
            let app = $handler::update_model(req, app);
            let app = app.update(&db).await?;
            $handler::after_update(&app)?;
            Ok(ApiResponse::success(app))
        }

        #[utoipa::path(
            delete,
            path= concat!("/api/admin/",$model_name,"/{id}"),
            security(("api_key" = [])),
            responses((status = 200, description = "Success", body = serde_json::Value))
        )]
        pub async fn delete(
            State(db): State<sea_orm::DatabaseConnection>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<()>, AppError> {
            $handler::before_delete(id)?;
            let app = <$entity>::find_by_id(id).one(&db).await?;
            let app = app.ok_or_else(|| AppError::not_found(stringify!($model_name).to_string(), Some(id)))?;
            crate::apply_delted!($fake_delete, app, &db);
            $handler::after_delete(id)?;
            Ok(ApiResponse::success(()))
        }

        #[utoipa::path(
            post,
            path = concat!("/api/admin/", $model_name, "/list"),
            security(("api_key" = [])),
            request_body = $search_payload,
            responses((status = 200, description = "Success", body = PagingResponse<$search_result>))
        )]
        pub async fn get_list(
            State(db): State<sea_orm::DatabaseConnection>,
            Query(params): Query<ListParamsReq>,
            Json(payload): Json<$search_payload>,
        ) -> Result<ApiResponse<PagingResponse<$search_result>>, AppError> {
            let page = params.page;
            let page_size = params.page_size;
            let query = $handler::build_query(payload)?;
            let paginator = query.paginate(&db, page_size);
            let total = paginator.num_items().await.unwrap_or(0);
            let list = paginator.fetch_page(page - 1).await?;
            let list = list.into_iter().filter_map(|item| <$search_result>::try_from(item).ok()).collect();
            Ok(ApiResponse::success(PagingResponse {
                list,
                total,
                page,
            }))
        }

        #[utoipa::path(
            get,
            path = concat!("/api/admin/", $model_name, "/{id}"),
            security(("api_key" = [])),
            responses((status = 200, description = "Success", body = $model))
        )]
        pub async fn get_by_id(
            State(db): State<sea_orm::DatabaseConnection>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<$search_result>, AppError> {
            let query = $handler::build_query_by_id(id)?;
            let app = query.one(&db).await?;
            let app = app.ok_or_else(|| AppError::not_found(stringify!($model_name).to_string(), Some(id)))?;
            let app = <$search_result>::try_from(app)?;
            Ok(ApiResponse::success(app))
        }
    };
}

// // 条件编译宏优化
// #[macro_export]
// macro_rules! apply_deleted_filter {
//     (true, $query:expr, $entity:ty) => {
//         $query.filter(<$entity as EntityTrait>::Column::DeletedAt.is_null())
//     };
//     (false, $query:expr, $entity:ty) => {
//         $query
//     };
// }

#[macro_export]
macro_rules! apply_delted {
    (true, $app:expr, $db:expr) => {
        let mut app = $app.into_active_model();
        app.deleted_at = Set(Some(Utc::now().naive_utc()));
        app.update($db).await?;
    };
    (false, $app:expr, $db:expr) => {
        $app.into_active_model().delete($db).await?;
    };
}

#[macro_export]
macro_rules! filter_if_some {
    // 匹配 .eq() 这种单参数的 filter 方法
    ($query:expr, $column:expr, $value:expr, eq) => {
        if let Some(val) = $value {
            $query = $query.filter($column.eq(val));
        }
    };
    // 匹配 .contains() 这种需要引用的 filter 方法
    ($query:expr, $column:expr, $value:expr, contains) => {
        if let Some(val) = $value.filter(|s| !s.is_empty()) {
            $query = $query.filter($column.contains(&val));
        }
    };
    // 你可以根据需要添加更多模式，例如 `gt`, `lt` 等
    ($query:expr, $column:expr, $value:expr, gt) => {
        if let Some(val) = $value {
            $query = $query.filter($column.gt(val));
        }
    };
}

#[macro_export]
macro_rules! update_field_if_some {
    // 普通赋值
    ($model:expr, $field:ident, $value:expr) => {
        if let Some(val) = $value {
            $model.$field = Set(val);
        }
    };
    // 需要特殊处理的赋值（例如哈希密码）
    ($model:expr, $field:ident, $value:expr, with $handler:expr) => {
        if let Some(val) = $value {
            $model.$field = Set($handler(val));
        }
    };
}
