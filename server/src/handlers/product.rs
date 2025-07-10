use crate::entities::product;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait};

/// Get all products
#[utoipa::path(
    get,
    path = "/api/admin/products",
    responses(
        (status = 200, description = "List of products", body = [product::Model])
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_products(
    State(db_pool): State<DatabaseConnection>,
) -> Result<Json<Vec<product::Model>>, StatusCode> {
    let products = product::Entity::find()
        .all(&db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}
