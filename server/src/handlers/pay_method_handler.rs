use crate::types::pay_method_types::*;
crate::import_crud_macro!();
use entity::pay_methods;
use salvo::{prelude::*, oapi::extract::JsonBody};
use salvo_oapi::extract::QueryParam;
// Create PayMethod
#[handler]
pub async fn add(
    depot: &mut Depot,
    req: JsonBody<PayMethodCreatePayload>,
) -> Result<ApiResponse<pay_methods::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = add_impl(&state, req.into_inner()).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(
    state: &AppState,
    req: PayMethodCreatePayload,
) -> Result<pay_methods::Model, AppError> {
    let active_model = pay_methods::ActiveModel {
        name: Set(req.name),
        remark: Set(req.remark),
        config: Set(req.config),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update PayMethod
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: QueryParam<i32>,
    req: JsonBody<PayMethodUpdatePayload>,
) -> Result<ApiResponse<pay_methods::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let pay_method = update_impl(&state, id.into_inner(), req.into_inner()).await?;
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
    let pay_method = pay_methods::Entity::find_by_id(id).one(&state.db).await?;
    let pay_method =
        pay_method.ok_or_else(|| AppError::not_found("pay_methods".to_string(), Some(id)))?;

    let mut pay_method = pay_method.into_active_model();
    pay_method.deleted_at = Set(Some(Utc::now()));
    pay_method.update(&state.db).await?;

    Ok(())
}

// Get PayMethods List
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    params: QueryParam<ListPayMethodsParams>,
) -> Result<ApiResponse<PagingResponse<pay_methods::Model>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let list = get_list_impl(&state, params.into_inner()).await?;
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
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: QueryParam<i32>,
) -> Result<ApiResponse<pay_methods::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let pay_method = get_by_id_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(pay_method))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<pay_methods::Model, AppError> {
    let query = pay_methods::Entity::find_by_id(id).one(&state.db).await?;
    let pay_method =
        query.ok_or_else(|| AppError::not_found("pay_methods".to_string(), Some(id)))?;
    Ok(pay_method)
}
