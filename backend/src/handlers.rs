use axum::{Json, extract::State, http::StatusCode};
use sqlx::SqlitePool;

use crate::models::Product;

pub async fn get_products(
    State(db_pool): State<SqlitePool>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}
