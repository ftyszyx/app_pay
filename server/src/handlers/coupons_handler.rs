use crate::types::common::{AppState, PagingResponse};
use crate::types::coupons_types::*;
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use salvo::{prelude::*, oapi::extract::JsonBody};
use salvo_oapi::extract::{PathParam};
use chrono::Utc;
use entity::coupons;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait,  PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

// Create Coupon
#[handler]
pub async fn add(
    depot: &mut Depot,
    req: JsonBody<CreateCouponReq>,
) -> Result<ApiResponse<coupons::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = add_impl(&state, req.into_inner()).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: CreateCouponReq) -> Result<coupons::Model, AppError> {
    let active_model = coupons::ActiveModel {
        code: Set(req.code),
        name: Set(req.name),
        status: Set(req.status),
        discount_type: Set(req.discount_type),
        discount_value: Set(req.discount_value),
        min_purchase_amount: Set(req.min_purchase_amount),
        start_time: Set(req.start_time),
        end_time: Set(req.end_time),
        usage_limit: Set(req.usage_limit),
        scope_type: Set(req.scope_type),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update Coupon
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: PathParam<i32>,
    req: JsonBody<UpdateCouponReq>,
) -> Result<ApiResponse<coupons::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let coupon = update_impl(&state, id.into_inner(), req.into_inner()).await?;
    Ok(ApiResponse::success(coupon))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: UpdateCouponReq,
) -> Result<coupons::Model, AppError> {
    let existing = coupons::Entity::find_by_id(id)
        .one(&state.db)
        .await?
        .ok_or(AppError::not_found("coupon", Some(id)))?;

    let mut active_model: coupons::ActiveModel = existing.into();

    crate::update_field_if_some!(active_model, code, req.code);
    crate::update_field_if_some!(active_model, name, req.name);
    crate::update_field_if_some!(active_model, status, req.status);
    crate::update_field_if_some!(active_model, discount_type, req.discount_type);
    crate::update_field_if_some!(active_model, discount_value, req.discount_value);
    crate::update_field_if_some!(active_model, min_purchase_amount, req.min_purchase_amount);
    crate::update_field_if_some!(active_model, start_time, req.start_time, option);
    crate::update_field_if_some!(active_model, end_time, req.end_time, option);
    crate::update_field_if_some!(active_model, usage_limit, req.usage_limit);
    crate::update_field_if_some!(active_model, scope_type, req.scope_type);

    active_model.updated_at = Set(Utc::now());

    let updated = active_model.update(&state.db).await?;
    Ok(updated)
}

// Get coupon by ID
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: PathParam<i32>,
) -> Result<ApiResponse<coupons::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let id_val = id.into_inner();
    let coupon = coupons::Entity::find_by_id(id_val)
        .one(&state.db)
        .await?
        .ok_or(AppError::not_found("coupon", Some(id_val)))?;
    Ok(ApiResponse::success(coupon))
}

// Delete coupon
#[handler]
pub async fn delete(
    depot: &mut Depot,
    id: PathParam<i32>,
) -> Result<ApiResponse<String>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let id_val = id.into_inner();
    let existing = coupons::Entity::find_by_id(id_val)
        .one(&state.db)
        .await?
        .ok_or(AppError::not_found("coupon", Some(id_val)))?;

    coupons::Entity::delete_by_id(existing.id)
        .exec(&state.db)
        .await?;

    Ok(ApiResponse::success("Deleted successfully".to_string()))
}

// Get coupons list with pagination and filtering
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<PagingResponse<coupons::Model>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let params = req.parse_queries::<SearchCouponsParams>()?;
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = coupons::Entity::find().order_by_desc(coupons::Column::CreatedAt);

    // Apply filters
    crate::filter_if_some!(query, coupons::Column::Id, params.id, eq);
    crate::filter_if_some!(query, coupons::Column::Code, params.code, contains);
    crate::filter_if_some!(query, coupons::Column::Name, params.name, contains);
    crate::filter_if_some!(query, coupons::Column::Status, params.status, eq);
    crate::filter_if_some!(
        query,
        coupons::Column::DiscountType,
        params.discount_type,
        eq
    );
    crate::filter_if_some!(query, coupons::Column::ScopeType, params.scope_type, eq);

    // Pagination
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;

    let response = PagingResponse { list, total, page };

    Ok(ApiResponse::success(response))
}
