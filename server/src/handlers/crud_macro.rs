#[macro_export]
macro_rules! import_crud_macro {
    () => {
        use crate::types::common::{AppError, ListParamsReq, PagingResponse};
        use crate::types::response::ApiResponse;
        use axum::{
            Json,
            extract::{Path, Query, State},
        };
        use chrono::Utc;
        use sea_orm::{
            ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
            QueryFilter, QueryOrder, Set,
        };
    };
}

#[macro_export]
macro_rules! impl_add_handler {
    (
        $model_name:ident,
        $entity:ty,
        $req_type:ty,
        $active_model:ty,
        $model:ty,
        $create_fn:ident
    ) => {
        #[utoipa::path( post,
                                    path = concat!("/api/admin/",stringify!($model_name)),
                                    security( ("api_key" = [])),
                                    request_body = $req_type,
                                    responses(
                                            (status = 200, description = "Success", body = $model),
                                        )
                                    )]
        pub async fn add(
            State(db): State<sea_orm::DatabaseConnection>,
            Json(req): Json<$req_type>,
        ) -> Result<ApiResponse<$model>, AppError> {
            let new_entity: $active_model = $create_fn(req);
            let entity = new_entity.insert(&db).await?;
            Ok(ApiResponse::success(entity))
        }
    };
}

#[macro_export]
macro_rules! impl_update_handler {
    (
        $model_name:ident,
        $entity:ty,
        $req_type:ty,
        $active_model:ty,
        $model:ty,
        $update_fn:ident
    ) => {
        #[utoipa::path(
                                    put,
                                    path = concat!("/api/admin/",stringify!($model_name),"/{id}"),
                                    security( ("api_key" = [])),
                                    request_body = $req_type,
                                    responses(
                                        (status = 200, description = "Success", body = $model),
                                    )
                                )]
        pub async fn update(
            State(db): State<sea_orm::DatabaseConnection>,
            Path(id): Path<i32>,
            Json(req): Json<$req_type>,
        ) -> Result<ApiResponse<$model>, AppError> {
            let app = <$entity>::find_by_id(id).one(&db).await?;
            let app = app.ok_or(AppError::DataNotFound)?;
            let app = $update_fn(req, app);
            let app = app.update(&db).await?;
            Ok(ApiResponse::success(app))
        }
    };
}

#[macro_export]
macro_rules! impl_delete_handler {
    (
        $model_name:ident,
        $entity:ty,
        $active_model:ty,
        $model:ty,
    ) => {
        #[utoipa::path(
                            delete,
                            path = concat!("/api/admin/",stringify!($model_name),"/{id}"),
                            security( ("api_key" = [])),
                            responses(
                                (status = 200, description = "Success", body = serde_json::Value),
                            )
                        )]
        pub async fn delete(
            State(db): State<sea_orm::DatabaseConnection>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<()>, AppError> {
            let app = <$entity>::find_by_id(id).one(&db).await?;
            let app = app.ok_or(AppError::DataNotFound)?;
            let app = app.delete(&db).await?;
            Ok(ApiResponse::success(()))
        }
    };
}

#[macro_export]
macro_rules! impl_fake_delete_handler {
    (
        $model_name:ident,
        $entity:ty,
        $active_model:ty,
        $model:ty
    ) => {
        #[utoipa::path(
                    delete,
                    path = concat!("/api/admin/",stringify!($model_name),"/{id}"),
                    security( ("api_key" = [])),
                    responses( (status = 200, description = "Success", body = serde_json::Value),
                    )
                )]
        pub async fn fake_delete(
            State(db): State<sea_orm::DatabaseConnection>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<()>, AppError> {
            let app = <$entity>::find_by_id(id).one(&db).await?;
            let app = app.ok_or(AppError::DataNotFound)?;
            let mut app: $active_model = app.into_active_model();
            app.deleted_at = Set(Some(Utc::now().naive_utc()));
             app.update(&db).await?;
            Ok(ApiResponse::success(()))
        }
    };
}

#[macro_export]
macro_rules! impl_get_handler {
    (
        $model_name:ident,
        $entity:ty,
        $req_type:ty,
        $model:ty,
        $query_fn:ident
    ) => {
        #[utoipa::path(
                    post,
                    path = concat!("/api/admin/",stringify!($model_name),"list"),
                    security( ("api_key" = [])),
                    responses(
                        (status = 200, description = "Success", body = PagingResponse<$model>),
                    )
                )]
        pub async fn get_list(
            State(db): State<sea_orm::DatabaseConnection>,
            Query(params): Query<ListParamsReq>,
            Json(payload): Json<$req_type>,
        ) -> Result<ApiResponse<PagingResponse<$model>>, AppError> {
            let page = params.page;
            let page_size = params.page_size;
            let query = $query_fn(payload);
            let paginator = query.paginate(&db, page_size);
            let total = paginator.num_items().await.unwrap_or(0);
            let apps = paginator.fetch_page(page - 1).await?;
            let list: Vec<$model> = apps.into_iter().collect();
            Ok(ApiResponse::success(PagingResponse {
                list,
                total,
                page,
            }))
        }
    };
}

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
macro_rules! impl_get_by_id_handler {
    (
        $model_name:ident,
        $entity:ty,
        $model:ty,
        $filter_deleted:tt // 改为 tt (token tree) 以支持 true/false 字面量
    ) => {
        #[utoipa::path(
                                    get,
                                    path = concat!("/api/admin/",stringify!($model_name),"/{id}"),
                                    security( ("api_key" = [])),
                                    responses(
                                        (status = 200, description = "Success", body = $model),
                                    )
                                )]
        pub async fn get_by_id(
            State(db): State<sea_orm::DatabaseConnection>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<$model>, AppError> {
              let query = crate::apply_deleted_filter!($filter_deleted, <$entity>::find_by_id(id), $entity);
            let app = query .one(&db) .await?;
             let app = app.ok_or(AppError::DataNotFound)?;
            Ok(ApiResponse::success(app))
        }
    };
}
