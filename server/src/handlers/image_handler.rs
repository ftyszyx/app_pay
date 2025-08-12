use crate::types::image_types::*;
crate::import_crud_macro!();
use entity::images;

// Create Image
#[utoipa::path(
    post,
    path = "/api/admin/images",
    security(("api_key" = [])),
    request_body = ImageCreatePayload,
    responses((status = 200, description = "Success", body = images::Model))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<ImageCreatePayload>,
) -> Result<ApiResponse<images::Model>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(
    state: &AppState,
    req: ImageCreatePayload,
) -> Result<images::Model, AppError> {
    let active_model = images::ActiveModel {
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

// Update Image
#[utoipa::path(
    put,
    path = "/api/admin/images/{id}",
    security(("api_key" = [])),
    request_body = ImageUpdatePayload,
    responses((status = 200, description = "Success", body = images::Model))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<ImageUpdatePayload>,
) -> Result<ApiResponse<images::Model>, AppError> {
    let entity = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: ImageUpdatePayload,
) -> Result<images::Model, AppError> {
    let model = images::Entity::find_by_id(id).one(&state.db).await?;
    let model = model.ok_or_else(|| AppError::not_found("images".to_string(), Some(id)))?;
    let mut model: images::ActiveModel = model.into_active_model();
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

// Delete Image (soft)
#[utoipa::path(
    delete,
    path = "/api/admin/images/{id}",
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
    let model = images::Entity::find_by_id(id).one(&state.db).await?;
    let model = model.ok_or_else(|| AppError::not_found("images".to_string(), Some(id)))?;
    let mut model: images::ActiveModel = model.into_active_model();
    crate::update_field_if_some!(model, deleted_at, Some(Utc::now()), option);
    model.update(&state.db).await?;
    Ok(())
}

// Get Images List
#[utoipa::path(
    get,
    path = "/api/admin/images/list",
    security(("api_key" = [])),
    params(ListImagesParams),
    responses((status = 200, description = "Success", body = PagingResponse<images::Model>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<ListImagesParams>,
) -> Result<ApiResponse<PagingResponse<images::Model>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: ListImagesParams,
) -> Result<PagingResponse<images::Model>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = images::Entity::find().order_by_desc(images::Column::CreatedAt);
    crate::filter_if_some!(query, images::Column::Id, params.id, eq);
    crate::filter_if_some!(query, images::Column::Name, params.name, contains);
    crate::filter_if_some!(query, images::Column::ObjectKey, params.object_key, contains);
    crate::filter_if_some!(query, images::Column::Url, params.url, contains);
    crate::filter_if_some!(query, images::Column::Path, params.path, contains);
    crate::filter_if_some!(query, images::Column::Status, params.status, eq);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;
    Ok(PagingResponse { list, total, page })
}

// Get Image by ID
#[utoipa::path(
    get,
    path = "/api/admin/images/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = images::Model))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<images::Model>, AppError> {
    let entity = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<images::Model, AppError> {
    let query = images::Entity::find_by_id(id).one(&state.db).await?;
    let model = query.ok_or_else(|| AppError::not_found("images".to_string(), Some(id)))?;
    Ok(model)
}


