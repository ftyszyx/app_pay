use crate::types::role_types::*;
use entity::roles;
crate::import_crud_macro!();

// Create Role
#[utoipa::path(
    post,
    path = "/api/admin/roles",
    security(("api_key" = [])),
    request_body = RoleCreatePayload,
    responses((status = 200, description = "Success", body = roles::Model))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<RoleCreatePayload>,
) -> Result<ApiResponse<roles::Model>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: RoleCreatePayload) -> Result<roles::Model, AppError> {
    let active_model = roles::ActiveModel {
        name: Set(req.name),
        remark: Set(req.remark),
        created_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update Role
#[utoipa::path(
    put,
    path = "/api/admin/roles/{id}",
    security(("api_key" = [])),
    request_body = RoleUpdatePayload,
    responses((status = 200, description = "Success", body = roles::Model))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<RoleUpdatePayload>,
) -> Result<ApiResponse<roles::Model>, AppError> {
    let role = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(role))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: RoleUpdatePayload,
) -> Result<roles::Model, AppError> {
    let role = roles::Entity::find_by_id(id).one(&state.db).await?;
    let role = role.ok_or_else(|| AppError::not_found("roles".to_string(), Some(id)))?;
    let mut role: roles::ActiveModel = role.into_active_model();
    crate::update_field_if_some!(role, name, req.name);
    let role = role.update(&state.db).await?;
    Ok(role)
}

// Delete Role
#[utoipa::path(
    delete,
    path = "/api/admin/roles/{id}",
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
    let role = roles::Entity::find_by_id(id).one(&state.db).await?;
    let role = role.ok_or_else(|| AppError::not_found("roles".to_string(), Some(id)))?;
    let mut role = role.into_active_model();
    role.deleted_at = Set(Some(Utc::now()));
    let _ = role.update(&state.db).await?;
    Ok(())
}

// Get Roles List
#[utoipa::path(
    get,
    path = "/api/admin/roles/list",
    security(("api_key" = [])),
    params(ListRolesParams),
    responses((status = 200, description = "Success", body = PagingResponse<roles::Model>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<ListRolesParams>,
) -> Result<ApiResponse<PagingResponse<roles::Model>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: ListRolesParams,
) -> Result<PagingResponse<roles::Model>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = roles::Entity::find()
        .filter(roles::Column::DeletedAt.is_null())
        .order_by_desc(roles::Column::CreatedAt);
    crate::filter_if_some!(query, roles::Column::Name, params.name, contains);
    crate::filter_if_some!(query, roles::Column::Id, params.id, eq);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;
    Ok(PagingResponse { list, total, page })
}

// Get Role by ID
#[utoipa::path(
    get,
    path = "/api/admin/roles/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = roles::Model))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<roles::Model>, AppError> {
    let role = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(role))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<roles::Model, AppError> {
    let query = roles::Entity::find_by_id(id).one(&state.db).await?;
    let role = query.ok_or_else(|| AppError::not_found("roles".to_string(), Some(id)))?;
    Ok(role)
}
