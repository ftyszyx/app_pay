use crate::entities::products;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait};

/// Get all products
#[utoipa::path(
    get,
    path = "/api/admin/products",
    responses(
        (status = 200, description = "List of products", body = [products::Model])
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_products(
    State(db_pool): State<DatabaseConnection>,
) -> Result<Json<Vec<products::Model>>, StatusCode> {
    let products = products::Entity::find()
        .all(&db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}
