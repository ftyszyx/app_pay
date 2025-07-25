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
        use crate::types::common::{  PagingResponse,AppState};
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
            State(state): State<AppState>,
            Json(req): Json<$create_payload>,
        ) -> Result<ApiResponse<$model>, AppError> {
            let entity = add_impl(&state,req).await?;
            Ok(ApiResponse::success(entity))
        }

        pub async fn add_impl(state:&AppState,req:$create_payload) -> Result<$model,AppError>{
            $handler::before_add(&req)?;
            let new_entity = $handler::create_model(req)?;
            let entity = new_entity.insert(&state.db).await?;
            $handler::after_add(&entity)?;
            Ok(entity)
        }

        #[utoipa::path(
            put,
            path = concat!("/api/admin/", $model_name, "/{id}"),
            security(("api_key" = [])),
            request_body = $update_payload,
            responses((status = 200, description = "Success", body = $model))
        )]
        pub async fn update(
            State(state): State<AppState>,
            Path(id): Path<i32>,
            Json(req): Json<$update_payload>,
        ) -> Result<ApiResponse<$model>, AppError> {
            let app = update_impl(&state,id,req).await?;
            Ok(ApiResponse::success(app))
        }

        pub async fn update_impl(state:&AppState,id:i32,req:$update_payload) -> Result<$model,AppError>{
            $handler::before_update(id, &req)?;
            let app = <$entity>::find_by_id(id).one(&state.db).await?;
            let app = app.ok_or_else(|| AppError::not_found(stringify!($model_name).to_string(), Some(id)))?;
            let app = $handler::update_model(req, app)?;
            let app = app.update(&state.db).await?;
            $handler::after_update(&app)?;
            Ok(app)
        }

        #[utoipa::path(
            delete,
            path= concat!("/api/admin/",$model_name,"/{id}"),
            security(("api_key" = [])),
            responses((status = 200, description = "Success", body = serde_json::Value))
        )]
        pub async fn delete(
            State(state): State<AppState>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<()>, AppError> {
            delete_impl(&state,id).await?;
            Ok(ApiResponse::success(()))
        }

        pub async fn delete_impl(state:&AppState,id:i32) -> Result<(),AppError>{
            $handler::before_delete(id)?;
            let app = <$entity>::find_by_id(id).one(&state.db).await?;
            let app = app.ok_or_else(|| AppError::not_found(stringify!($model_name).to_string(), Some(id)))?;
            crate::apply_delted!($fake_delete, app, &state.db);
            $handler::after_delete(id)?;
            Ok(())
        }

        #[utoipa::path(
            get,
            path = concat!("/api/admin/", $model_name, "/list"),
            security(("api_key" = [])),
            params($search_payload),
            responses((status = 200, description = "Success", body = PagingResponse<$search_result>))
        )]
        pub async fn get_list(
            State(state): State<AppState>,
            Query(params): Query<$search_payload>,
        ) -> Result<ApiResponse<PagingResponse<$search_result>>, AppError> {
            let list = get_list_impl(&state,params).await?;
            Ok(ApiResponse::success(list))
        }

        pub async fn get_list_impl(state:&AppState,params:$search_payload) -> Result<PagingResponse<$search_result>,AppError>{
            let page = params.pagination.page.unwrap_or(1);
            let page_size = params.pagination.page_size.unwrap_or(20);
            let query = $handler::build_query(params)?;
            let paginator = query.paginate(&state.db, page_size);
            let total = paginator.num_items().await.unwrap_or(0);
            let list = paginator.fetch_page(page - 1).await?;
            let list = list.into_iter().filter_map(|item| <$search_result>::try_from(item).ok()).collect();
            Ok(PagingResponse {
                list,
                total,
                page,
            })
        }

        #[utoipa::path(
            get,
            path = concat!("/api/admin/", $model_name, "/{id}"),
            security(("api_key" = [])),
            responses((status = 200, description = "Success", body = $model))
        )]
        pub async fn get_by_id(
            State(state): State<AppState>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<$search_result>, AppError> {
            let app = get_by_id_impl(&state,id).await?;
            Ok(ApiResponse::success(app))
        }

        pub async fn get_by_id_impl(state:&AppState,id:i32) -> Result<$search_result,AppError>{
            let query = $handler::build_query_by_id(id)?;
            let app = query.one(&state.db).await?;
            let app = app.ok_or_else(|| AppError::not_found(stringify!($model_name).to_string(), Some(id)))?;
            let app = <$search_result>::try_from(app)?;
            Ok(app)
        }
    };
}

#[macro_export]
macro_rules! apply_delted {
    (true, $app:expr, $db:expr) => {
        let mut app = $app.into_active_model();
        app.deleted_at = Set(Some(Utc::now()));
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
    // -> 新增：处理 Option<T> 字段
    ($model:expr, $field:ident, $value:expr, option) => {
        if let Some(val) = $value {
            $model.$field = Set(Some(val));
        }
    };
    // 需要特殊处理的赋值（例如哈希密码）
    ($model:expr, $field:ident, $value:expr, with $handler:expr) => {
        if let Some(val) = $value {
            $model.$field = Set($handler(val));
        }
    };
}
