use crate::types::product_types::*;
crate::import_crud_macro!();
use entity::products;
use salvo::{prelude::*, oapi::extract::JsonBody};
use salvo_oapi::extract::QueryParam;

// Create Product
#[handler]
pub async fn add(
    depot: &mut Depot,
    req: JsonBody<ProductCreatePayload>,
) -> Result<ApiResponse<products::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = add_impl(&state, req.into_inner()).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(
    state: &AppState,
    req: ProductCreatePayload,
) -> Result<products::Model, AppError> {
    let active_model = products::ActiveModel {
        name: Set(req.name),
        price: Set(req.price),
        app_id: Set(req.app_id),
        product_id: Set(req.product_id),
        add_valid_days: Set(req.add_valid_days),
        image_url: Set(req.image_url),
        tags: Set(req.tags),
        status: Set(req.status),
        remark: Set(req.remark),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update Product
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: QueryParam<i32>,
    req: JsonBody<ProductUpdatePayload>,
) -> Result<ApiResponse<products::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let product = update_impl(&state, id.into_inner(), req.into_inner()).await?;
    Ok(ApiResponse::success(product))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: ProductUpdatePayload,
) -> Result<products::Model, AppError> {
    let product = products::Entity::find_by_id(id).one(&state.db).await?;
    let product = product.ok_or_else(|| AppError::not_found("products".to_string(), Some(id)))?;
    let mut product: products::ActiveModel = product.into_active_model();
    crate::update_field_if_some!(product, name, req.name);
    crate::update_field_if_some!(product, price, req.price);
    crate::update_field_if_some!(product, app_id, req.app_id);
    crate::update_field_if_some!(product, product_id, req.product_id);
    crate::update_field_if_some!(product, add_valid_days, req.add_valid_days);
    crate::update_field_if_some!(product, image_url, req.image_url, option);
    crate::update_field_if_some!(product, tags, req.tags, option);
    crate::update_field_if_some!(product, remark, req.remark, option);
    crate::update_field_if_some!(product, status, req.status);
    let product = product.update(&state.db).await?;
    Ok(product)
}

// Delete Product
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
    let product = products::Entity::find_by_id(id).one(&state.db).await?;
    let product = product.ok_or_else(|| AppError::not_found("products".to_string(), Some(id)))?;
    let mut product: products::ActiveModel = product.into_active_model();
    crate::update_field_if_some!(product, deleted_at, Some(Utc::now()), option);
    product.update(&state.db).await?;
    Ok(())
}

// Get Products List
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    params: QueryParam<ListProductsParams>,
) -> Result<ApiResponse<PagingResponse<products::Model>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let list = get_list_impl(&state, params.into_inner()).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: ListProductsParams,
) -> Result<PagingResponse<products::Model>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = products::Entity::find().order_by_desc(products::Column::CreatedAt);
    crate::filter_if_some!(query, products::Column::Id, params.id, eq);
    crate::filter_if_some!(
        query,
        products::Column::ProductId,
        params.product_id,
        contains
    );
    crate::filter_if_some!(query, products::Column::Name, params.name, contains);
    crate::filter_if_some!(query, products::Column::Status, params.status, eq);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let list = paginator.fetch_page(page - 1).await?;
    Ok(PagingResponse { list, total, page })
}

// Get Product by ID
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: QueryParam<i32>,
) -> Result<ApiResponse<products::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let product = get_by_id_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(product))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<products::Model, AppError> {
    let query = products::Entity::find_by_id(id).one(&state.db).await?;
    let product = query.ok_or_else(|| AppError::not_found("products".to_string(), Some(id)))?;
    Ok(product)
}
