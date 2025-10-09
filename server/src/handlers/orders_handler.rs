use crate::types::orders_types::*;
use entity::orders;
crate::import_crud_macro!();
use salvo::{prelude::*, oapi::extract::JsonBody};
use salvo_oapi::extract::{PathParam};

#[handler]
pub async fn add(
    depot: &mut Depot,
    req: JsonBody<CreateOrderReq>,
) -> Result<ApiResponse<orders::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = add_impl(&state, req.into_inner()).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: CreateOrderReq) -> Result<orders::Model, AppError> {
    let active_model = orders::ActiveModel {
        order_id: Set(req.order_id),
        user_info: Set(req.user_info),
        status: Set(req.status),
        pay_method_id: Set(req.pay_method_id),
        original_price: Set(req.original_price),
        final_price: Set(req.final_price),
        remark: Set(req.remark),
        created_by: Set(req.created_by),
        updated_by: Set(req.updated_by),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update Order
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: PathParam<i32>,
    req: JsonBody<UpdateOrderReq>,
) -> Result<ApiResponse<orders::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let order = update_impl(&state, id.into_inner(), req.into_inner()).await?;
    Ok(ApiResponse::success(order))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: UpdateOrderReq,
) -> Result<orders::Model, AppError> {
    let order = orders::Entity::find_by_id(id).one(&state.db).await?;
    let order = order.ok_or_else(|| AppError::not_found("orders".to_string(), Some(id)))?;
    let mut order: orders::ActiveModel = order.into_active_model();
    crate::update_field_if_some!(order, order_id, req.order_id);
    crate::update_field_if_some!(order, user_info, req.user_info, option);
    crate::update_field_if_some!(order, status, req.status);
    crate::update_field_if_some!(order, pay_method_id, req.pay_method_id);
    crate::update_field_if_some!(order, original_price, req.original_price);
    crate::update_field_if_some!(order, final_price, req.final_price);
    crate::update_field_if_some!(order, remark, req.remark, option);
    crate::update_field_if_some!(order, updated_by, req.updated_by);
    let order = order.update(&state.db).await?;
    Ok(order)
}

// Delete Order
#[handler]
pub async fn delete(
    depot: &mut Depot,
    id: PathParam<i32>,
) -> Result<ApiResponse<()>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    delete_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(()))
}

pub async fn delete_impl(state: &AppState, id: i32) -> Result<(), AppError> {
    let order = orders::Entity::find_by_id(id).one(&state.db).await?;
    let order = order.ok_or_else(|| AppError::not_found("orders".to_string(), Some(id)))?;
    order.into_active_model().delete(&state.db).await?;
    Ok(())
}

// Get Orders List
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<PagingResponse<OrderInfo>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let params = req.parse_queries::<SearchOrdersParams>()?;
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchOrdersParams,
) -> Result<PagingResponse<OrderInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = orders::Entity::find().order_by_desc(orders::Column::CreatedAt);
    crate::filter_if_some!(query, orders::Column::Id, params.id, eq);
    crate::filter_if_some!(query, orders::Column::OrderId, params.order_id, contains);
    crate::filter_if_some!(query, orders::Column::Status, params.status, eq);
    crate::filter_if_some!(query, orders::Column::PayMethodId, params.pay_method_id, eq);
    crate::filter_if_some!(query, orders::Column::CreatedBy, params.created_by, eq);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;
    let list = list
        .into_iter()
        .filter_map(|item| OrderInfo::try_from(item).ok())
        .collect();
    Ok(PagingResponse { list, total, page })
}

// Get Order by ID
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: PathParam<i32>,
) -> Result<ApiResponse<OrderInfo>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let id_val = id.into_inner();
    let query = orders::Entity::find_by_id(id_val).one(&state.db).await?;
    let order = query.ok_or_else(|| AppError::not_found("orders".to_string(), Some(id_val)))?;
    let order = OrderInfo::try_from(order)?;
    Ok(ApiResponse::success(order))
}
