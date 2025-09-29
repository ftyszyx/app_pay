use crate::types::common::{AppState, PagingResponse};
use crate::types::error::AppError;
use crate::types::resource_types::*;
use crate::types::response::ApiResponse;
use salvo::{prelude::*, oapi::extract::JsonBody};
use salvo_oapi::extract::QueryParam;
use entity::resources;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

// Create Resource
#[handler]
pub async fn add(
    depot: &mut Depot,
    req: JsonBody<ResourceCreatePayload>,
) -> Result<ApiResponse<resources::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = add_impl(&state, req.into_inner()).await?;
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
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: QueryParam<i32>,
    req: JsonBody<ResourceUpdatePayload>,
) -> Result<ApiResponse<resources::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = update_impl(&state, id.into_inner(), req.into_inner()).await?;
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
#[handler]
pub async fn delete(
    depot: &mut Depot,
    id: QueryParam<i32>,
) -> Result<ApiResponse<()>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    delete_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(()))
}

pub async fn delete_impl(state: &AppState, id: i32) -> Result<(), AppError> {
    let model = resources::Entity::find_by_id(id).one(&state.db).await?;
    let model = model.ok_or_else(|| AppError::not_found("resources".to_string(), Some(id)))?;
    model.delete(&state.db).await?;
    Ok(())
}

// Get Resources List
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    params: QueryParam<ListResourcesParams>,
) -> Result<ApiResponse<PagingResponse<resources::Model>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let list = get_list_impl(&state, params.into_inner()).await?;
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
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: QueryParam<i32>,
) -> Result<ApiResponse<resources::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = get_by_id_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<resources::Model, AppError> {
    let query = resources::Entity::find_by_id(id).one(&state.db).await?;
    let model = query.ok_or_else(|| AppError::not_found("resources".to_string(), Some(id)))?;
    Ok(model)
}
