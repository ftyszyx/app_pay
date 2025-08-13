use crate::types::common::{AppState, PagingResponse};
use crate::types::error::AppError;
use crate::types::resource_types::*;
use crate::types::response::ApiResponse;
use axum::{
    Json,
    extract::{Path, Query, State},
};
use entity::resources;
use sea_orm::{
    ModelTrait,
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

// Create Resource
#[utoipa::path(
    post,
    path = "/api/admin/resources",
    security(("api_key" = [])),
    request_body = ResourceCreatePayload,
    responses((status = 200, description = "Success", body = resources::Model))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<ResourceCreatePayload>,
) -> Result<ApiResponse<resources::Model>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(
    state: &AppState,
    req: ResourceCreatePayload,
) -> Result<resources::Model, AppError> {
    let active_model = resources::ActiveModel {
        name: Set(req.name),
        object_key: Set(req.object_key),
        url: Set(req.url),
        path: Set(req.path),
        tags: Set(req.tags),
        status: Set(req.status),
        remark: Set(req.remark),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update Resource
#[utoipa::path(
    put,
    path = "/api/admin/resources/{id}",
    security(("api_key" = [])),
    request_body = ResourceUpdatePayload,
    responses((status = 200, description = "Success", body = resources::Model))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<ResourceUpdatePayload>,
) -> Result<ApiResponse<resources::Model>, AppError> {
    let entity = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: ResourceUpdatePayload,
) -> Result<resources::Model, AppError> {
    let model = resources::Entity::find_by_id(id).one(&state.db).await?;
    let model = model.ok_or_else(|| AppError::not_found("resources".to_string(), Some(id)))?;
    let mut model: resources::ActiveModel = model.into_active_model();
    crate::update_field_if_some!(model, name, req.name);
    crate::update_field_if_some!(model, object_key, req.object_key);
    crate::update_field_if_some!(model, url, req.url);
    crate::update_field_if_some!(model, path, req.path);
    crate::update_field_if_some!(model, tags, req.tags, option);
    crate::update_field_if_some!(model, remark, req.remark, option);
    crate::update_field_if_some!(model, status, req.status);
    let model = model.update(&state.db).await?;
    Ok(model)
}

// Delete Resource (soft)
#[utoipa::path(
    delete,
    path = "/api/admin/resources/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = serde_json::Value))
)]
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<()>, AppError> {
    delete_impl(&state, id).await?;
    Ok(ApiResponse::success(()))
}

pub async fn delete_impl(state: &AppState, id: i32) -> Result<(), AppError> {
    let model = resources::Entity::find_by_id(id).one(&state.db).await?;
    let model = model.ok_or_else(|| AppError::not_found("resources".to_string(), Some(id)))?;
    model.delete(&state.db).await?;
    Ok(())
}

// Get Resources List
#[utoipa::path(
    get,
    path = "/api/admin/resources/list",
    security(("api_key" = [])),
    params(ListResourcesParams),
    responses((status = 200, description = "Success", body = PagingResponse<resources::Model>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<ListResourcesParams>,
) -> Result<ApiResponse<PagingResponse<resources::Model>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: ListResourcesParams,
) -> Result<PagingResponse<resources::Model>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = resources::Entity::find().order_by_desc(resources::Column::CreatedAt);
    crate::filter_if_some!(query, resources::Column::Id, params.id, eq);
    crate::filter_if_some!(query, resources::Column::Name, params.name, contains);
    crate::filter_if_some!(
        query,
        resources::Column::ObjectKey,
        params.object_key,
        contains
    );
    crate::filter_if_some!(query, resources::Column::Url, params.url, contains);
    crate::filter_if_some!(query, resources::Column::Path, params.path, contains);
    crate::filter_if_some!(query, resources::Column::Status, params.status, eq);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;
    Ok(PagingResponse { list, total, page })
}

// Get Resource by ID
#[utoipa::path(
    get,
    path = "/api/admin/resources/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = resources::Model))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<resources::Model>, AppError> {
    let entity = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<resources::Model, AppError> {
    let query = resources::Entity::find_by_id(id).one(&state.db).await?;
    let model = query.ok_or_else(|| AppError::not_found("resources".to_string(), Some(id)))?;
    Ok(model)
}
