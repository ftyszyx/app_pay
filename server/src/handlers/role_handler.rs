use crate::types::role_types::*;
use entity::roles;
crate::import_crud_macro!();
use salvo::{prelude::*, oapi::extract::JsonBody};
use salvo_oapi::extract::QueryParam;

// Create Role
#[handler]
pub async fn add(
    depot: &mut Depot,
    req: JsonBody<RoleCreatePayload>,
) -> Result<ApiResponse<roles::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = add_impl(&state, req.into_inner()).await?;
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
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: QueryParam<i32>,
    req: JsonBody<RoleUpdatePayload>,
) -> Result<ApiResponse<roles::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let role = update_impl(&state, id.into_inner(), req.into_inner()).await?;
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
    let role = roles::Entity::find_by_id(id).one(&state.db).await?;
    let role = role.ok_or_else(|| AppError::not_found("roles".to_string(), Some(id)))?;
    let mut role = role.into_active_model();
    role.deleted_at = Set(Some(Utc::now()));
    let _ = role.update(&state.db).await?;
    Ok(())
}

// Get Roles List
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    params: QueryParam<ListRolesParams>,
) -> Result<ApiResponse<PagingResponse<roles::Model>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let list = get_list_impl(&state, params.into_inner()).await?;
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
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: QueryParam<i32>,
) -> Result<ApiResponse<roles::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let role = get_by_id_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(role))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<roles::Model, AppError> {
    let query = roles::Entity::find_by_id(id).one(&state.db).await?;
    let role = query.ok_or_else(|| AppError::not_found("roles".to_string(), Some(id)))?;
    Ok(role)
}
