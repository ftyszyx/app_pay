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
        $list_payload:ty,
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
            request_body = $list_payload,
            responses((status = 200, description = "Success", body = PagingResponse<$model>))
        )]
        pub async fn get_list(
            State(db): State<sea_orm::DatabaseConnection>,
            Query(params): Query<ListParamsReq>,
            Json(payload): Json<$list_payload>,
        ) -> Result<ApiResponse<PagingResponse<$model>>, AppError> {
            let page = params.page;
            let page_size = params.page_size;
            let query = $handler::build_query(payload);
            let paginator = query.paginate(&db, page_size);
            let total = paginator.num_items().await.unwrap_or(0);
            let list = paginator.fetch_page(page - 1).await?;
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
        ) -> Result<ApiResponse<<$entity as EntityTrait>::Model>, AppError> {
            let query = crate::apply_deleted_filter!($fake_delete, <$entity>::find_by_id(id), $entity);
            let app = query.one(&db).await?;
            let app = app.ok_or_else(|| AppError::not_found(stringify!($model_name).to_string(), Some(id)))?;
            Ok(ApiResponse::success(app))
        }
    };
}

// 条件编译宏优化
#[macro_export]
macro_rules! apply_deleted_filter {
    (true, $query:expr, $entity:ty) => {
        $query.filter(<$entity as EntityTrait>::Column::DeletedAt.is_null())
    };
    (false, $query:expr, $entity:ty) => {
        $query
    };
}

#[macro_export]
macro_rules! apply_delted {
    (true, $app:expr, $db:expr) => {
            let mut app= $app.into_active_model();
            app.deleted_at = Set(Some(Utc::now().naive_utc()));
            app.update($db).await?;
    };
    (false, $app:expr, $db:expr) => {
            $app.into_active_model().delete($db).await?;
    };
}