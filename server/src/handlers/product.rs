use crate::handlers::response::ApiResponse;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use entity::products;
use sea_orm::{DatabaseConnection, EntityTrait};

/// Get all products
#[utoipa::path(
    get,
    path = "/api/admin/products",
    responses(
        (status = 200, description = "List of products", body = ApiResponse<Vec<products::Model>>)
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_products(
    State(db_pool): State<DatabaseConnection>,
) -> impl IntoResponse {
    let products_result = products::Entity::find().all(&db_pool).await;

    match products_result {
        Ok(products) => ApiResponse::success(products),
        Err(e) => ApiResponse::<Vec<products::Model>>::error(
            StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        ),
    }
}
