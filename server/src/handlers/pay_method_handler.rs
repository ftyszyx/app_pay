use crate::types::pay_method_types::*;
crate::import_crud_macro!();
use entity::pay_methods;
// Create PayMethod
#[utoipa::path(
    post,
    path = "/api/admin/pay_methods",
    security(("api_key" = [])),
    request_body = PayMethodCreatePayload,
    responses((status = 200, description = "Success", body = pay_methods::Model))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<PayMethodCreatePayload>,
) -> Result<ApiResponse<pay_methods::Model>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(
    state: &AppState,
    req: PayMethodCreatePayload,
) -> Result<pay_methods::Model, AppError> {
    let active_model = pay_methods::ActiveModel {
        name: Set(req.name),
        status: Set(req.status),
        remark: Set(req.remark),
        config: Set(req.config),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update PayMethod
#[utoipa::path(
    put,
    path = "/api/admin/pay_methods/{id}",
    security(("api_key" = [])),
    request_body = PayMethodUpdatePayload,
    responses((status = 200, description = "Success", body = pay_methods::Model))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<PayMethodUpdatePayload>,
) -> Result<ApiResponse<pay_methods::Model>, AppError> {
    let pay_method = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(pay_method))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: PayMethodUpdatePayload,
) -> Result<pay_methods::Model, AppError> {
    let pay_method = pay_methods::Entity::find_by_id(id).one(&state.db).await?;
    let pay_method =
        pay_method.ok_or_else(|| AppError::not_found("pay_methods".to_string(), Some(id)))?;
    let mut pay_method: pay_methods::ActiveModel = pay_method.into_active_model();
    crate::update_field_if_some!(pay_method, name, req.name);
    let pay_method = pay_method.update(&state.db).await?;
    Ok(pay_method)
}

// Delete PayMethod
#[utoipa::path(
    delete,
    path = "/api/admin/pay_methods/{id}",
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
    let pay_method = pay_methods::Entity::find_by_id(id).one(&state.db).await?;
    let pay_method =
        pay_method.ok_or_else(|| AppError::not_found("pay_methods".to_string(), Some(id)))?;

    let mut pay_method = pay_method.into_active_model();
    pay_method.deleted_at = Set(Some(Utc::now()));
    pay_method.update(&state.db).await?;

    Ok(())
}

// Get PayMethods List
#[utoipa::path(
    get,
    path = "/api/admin/pay_methods/list",
    security(("api_key" = [])),
    params(ListPayMethodsParams),
    responses((status = 200, description = "Success", body = PagingResponse<pay_methods::Model>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<ListPayMethodsParams>,
) -> Result<ApiResponse<PagingResponse<pay_methods::Model>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: ListPayMethodsParams,
) -> Result<PagingResponse<pay_methods::Model>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = pay_methods::Entity::find().order_by_desc(pay_methods::Column::CreatedAt);
    crate::filter_if_some!(query, pay_methods::Column::Name, params.name, contains);
    crate::filter_if_some!(query, pay_methods::Column::Id, params.id, eq);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;
    Ok(PagingResponse { list, total, page })
}

// Get PayMethod by ID
#[utoipa::path(
    get,
    path = "/api/admin/pay_methods/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = pay_methods::Model))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<pay_methods::Model>, AppError> {
    let pay_method = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(pay_method))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<pay_methods::Model, AppError> {
    let query = pay_methods::Entity::find_by_id(id).one(&state.db).await?;
    let pay_method =
        query.ok_or_else(|| AppError::not_found("pay_methods".to_string(), Some(id)))?;
    Ok(pay_method)
}
