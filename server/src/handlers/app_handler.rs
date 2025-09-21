use crate::types::app_types::*;
use entity::apps;
crate::import_crud_macro!();

// Create App
#[utoipa::path(
    post,
    path = "/api/admin/apps",
    security(("api_key" = [])),
    request_body = AddAppReq,
    responses((status = 200, description = "Success", body = apps::Model))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<AddAppReq>,
) -> Result<ApiResponse<apps::Model>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: AddAppReq) -> Result<apps::Model, AppError> {
    let active_model = apps::ActiveModel {
        name: Set(req.name),
        app_id: Set(req.app_id),
        app_vername: Set(req.app_vername),
        app_vercode: Set(req.app_vercode),
        app_download_url: Set(req.app_download_url),
        app_res_url: Set(req.app_res_url),
        app_update_info: Set(req.app_update_info),
        app_valid_key: Set(req.app_valid_key.unwrap_or_default()),
        trial_days: Set(req.trial_days.unwrap_or_default()),
        sort_order: Set(req.sort_order),
        created_at: Set(Utc::now()),
        status: Set(req.status),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update App
#[utoipa::path(
    put,
    path = "/api/admin/apps/{id}",
    security(("api_key" = [])),
    request_body = UpdateAppReq,
    responses((status = 200, description = "Success", body = apps::Model))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateAppReq>,
) -> Result<ApiResponse<apps::Model>, AppError> {
    let app = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(app))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: UpdateAppReq,
) -> Result<apps::Model, AppError> {
    let app = apps::Entity::find_by_id(id).one(&state.db).await?;
    let app = app.ok_or_else(|| AppError::not_found("apps".to_string(), Some(id)))?;
    let mut app: apps::ActiveModel = app.into_active_model();
    crate::update_field_if_some!(app, name, req.name);
    crate::update_field_if_some!(app, app_id, req.app_id);
    crate::update_field_if_some!(app, app_vername, req.app_vername);
    crate::update_field_if_some!(app, app_vercode, req.app_vercode);
    crate::update_field_if_some!(app, app_download_url, req.app_download_url);
    crate::update_field_if_some!(app, app_res_url, req.app_res_url);
    crate::update_field_if_some!(app, app_update_info, req.app_update_info, option);
    crate::update_field_if_some!(app, app_valid_key, req.app_valid_key);
    crate::update_field_if_some!(app, trial_days, req.trial_days);
    crate::update_field_if_some!(app, sort_order, req.sort_order);
    crate::update_field_if_some!(app, status, req.status);
    let app = app.update(&state.db).await?;
    Ok(app)
}

// Delete App
#[utoipa::path(
    delete,
    path = "/api/admin/apps/{id}",
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
    let app = apps::Entity::find_by_id(id).one(&state.db).await?;
    let app = app.ok_or_else(|| AppError::not_found("apps".to_string(), Some(id)))?;
    let mut app = app.into_active_model();
    app.deleted_at = Set(Some(Utc::now()));
    let _ = app.update(&state.db).await?;
    Ok(())
}

// Get Apps List
#[utoipa::path(
    get,
    path = "/api/admin/apps/list",
    security(("api_key" = [])),
    params(ListAppsParams),
    responses((status = 200, description = "Success", body = PagingResponse<apps::Model>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<ListAppsParams>,
) -> Result<ApiResponse<PagingResponse<apps::Model>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: ListAppsParams,
) -> Result<PagingResponse<apps::Model>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = apps::Entity::find()
        .filter(apps::Column::DeletedAt.is_null())
        .order_by_desc(apps::Column::CreatedAt);
    crate::filter_if_some!(query, apps::Column::Name, params.name, contains);
    crate::filter_if_some!(query, apps::Column::Id, params.id, eq);
    crate::filter_if_some!(query, apps::Column::AppId, params.app_id, contains);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;
    Ok(PagingResponse { list, total, page })
}

// Get App by ID
#[utoipa::path(
    get,
    path = "/api/admin/apps/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = apps::Model))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<apps::Model>, AppError> {
    let app = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(app))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<apps::Model, AppError> {
    let query = apps::Entity::find_by_id(id).one(&state.db).await?;
    let app = query.ok_or_else(|| AppError::not_found("apps".to_string(), Some(id)))?;
    Ok(app)
}
