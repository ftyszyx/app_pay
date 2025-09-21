use crate::types::reg_codes_types::*;
crate::import_crud_macro!();
use entity::{apps, reg_codes};

// Create RegCode
#[utoipa::path(
    post,
    path = "/api/admin/reg_codes",
    security(("api_key" = [])),
    request_body = CreateRegCodeReq,
    responses((status = 200, description = "Success", body = RegCodeInfo))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<CreateRegCodeReq>,
) -> Result<ApiResponse<RegCodeInfo>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: CreateRegCodeReq) -> Result<RegCodeInfo, AppError> {
    let active_model = reg_codes::ActiveModel {
        code: Set(req.code),
        app_id: Set(req.app_id),
        bind_device_info: Set(req.bind_device_info),
        valid_days: Set(req.valid_days),
        max_devices: Set(req.max_devices),
        status: Set(req.status),
        code_type: Set(req.code_type),
        expire_time: Set(req.expire_time),
        total_count: Set(req.total_count),
        use_count: Set(0),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;

    // Fetch with app information for response
    let result = reg_codes::Entity::find_by_id(entity.id)
        .find_also_related(apps::Entity)
        .one(&state.db)
        .await?;

    match result {
        Some((reg_code, app)) => Ok(RegCodeInfo::try_from((reg_code, app))?),
        None => Err(AppError::not_found(
            "reg_codes".to_string(),
            Some(entity.id),
        )),
    }
}

// Update RegCode
#[utoipa::path(
    put,
    path = "/api/admin/reg_codes/{id}",
    security(("api_key" = [])),
    request_body = UpdateRegCodeReq,
    responses((status = 200, description = "Success", body = RegCodeInfo))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateRegCodeReq>,
) -> Result<ApiResponse<RegCodeInfo>, AppError> {
    let reg_code = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(reg_code))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: UpdateRegCodeReq,
) -> Result<RegCodeInfo, AppError> {
    let reg_code = reg_codes::Entity::find_by_id(id).one(&state.db).await?;
    let reg_code =
        reg_code.ok_or_else(|| AppError::not_found("reg_codes".to_string(), Some(id)))?;

    let mut reg_code: reg_codes::ActiveModel = reg_code.into_active_model();
    crate::update_field_if_some!(reg_code, code, req.code);
    crate::update_field_if_some!(reg_code, app_id, req.app_id);
    crate::update_field_if_some!(reg_code, bind_device_info, req.bind_device_info, option);
    crate::update_field_if_some!(reg_code, valid_days, req.valid_days);
    crate::update_field_if_some!(reg_code, max_devices, req.max_devices);
    crate::update_field_if_some!(reg_code, status, req.status);
    crate::update_field_if_some!(reg_code, binding_time, req.binding_time, option);
    crate::update_field_if_some!(reg_code, code_type, req.code_type);
    crate::update_field_if_some!(reg_code, expire_time, req.expire_time, option);
    crate::update_field_if_some!(reg_code, total_count, req.total_count, option);
    crate::update_field_if_some!(reg_code, use_count, req.use_count);
    crate::update_field_if_some!(reg_code, device_id, req.device_id, option);
    reg_code.updated_at = Set(Utc::now());

    let updated_reg_code = reg_code.update(&state.db).await?;

    // Fetch with app information for response
    let result = reg_codes::Entity::find_by_id(updated_reg_code.id)
        .find_also_related(apps::Entity)
        .one(&state.db)
        .await?;

    match result {
        Some((reg_code, app)) => Ok(RegCodeInfo::try_from((reg_code, app))?),
        None => Err(AppError::not_found(
            "reg_codes".to_string(),
            Some(updated_reg_code.id),
        )),
    }
}

// Delete RegCode
#[utoipa::path(
    delete,
    path = "/api/admin/reg_codes/{id}",
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
    let reg_code = reg_codes::Entity::find_by_id(id).one(&state.db).await?;
    let reg_code =
        reg_code.ok_or_else(|| AppError::not_found("reg_codes".to_string(), Some(id)))?;
    reg_code.into_active_model().delete(&state.db).await?;
    Ok(())
}

// Get RegCodes List
#[utoipa::path(
    get,
    path = "/api/admin/reg_codes/list",
    security(("api_key" = [])),
    params(SearchRegCodesParams),
    responses((status = 200, description = "Success", body = PagingResponse<RegCodeInfo>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<SearchRegCodesParams>,
) -> Result<ApiResponse<PagingResponse<RegCodeInfo>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchRegCodesParams,
) -> Result<PagingResponse<RegCodeInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);

    let mut query = reg_codes::Entity::find()
        .find_also_related(apps::Entity)
        .order_by_desc(reg_codes::Column::CreatedAt);

    crate::filter_if_some!(query, reg_codes::Column::Id, params.id, eq);
    crate::filter_if_some!(query, reg_codes::Column::Code, params.code, contains);
    crate::filter_if_some!(query, reg_codes::Column::AppId, params.app_id, eq);
    crate::filter_if_some!(query, reg_codes::Column::Status, params.status, eq);
    crate::filter_if_some!(query, reg_codes::Column::CodeType, params.code_type, eq);

    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let results = paginator.fetch_page(page - 1).await?;

    let list: Result<Vec<RegCodeInfo>, AppError> = results
        .into_iter()
        .map(|(reg_code, app)| RegCodeInfo::try_from((reg_code, app)))
        .collect();

    Ok(PagingResponse {
        list: list?,
        total,
        page,
    })
}

// Get RegCode by ID
#[utoipa::path(
    get,
    path = "/api/admin/reg_codes/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = RegCodeInfo))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<RegCodeInfo>, AppError> {
    let reg_code = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(reg_code))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<RegCodeInfo, AppError> {
    let result = reg_codes::Entity::find_by_id(id)
        .find_also_related(apps::Entity)
        .one(&state.db)
        .await?;

    match result {
        Some((reg_code, app)) => Ok(RegCodeInfo::try_from((reg_code, app))?),
        None => Err(AppError::not_found("reg_codes".to_string(), Some(id))),
    }
}
