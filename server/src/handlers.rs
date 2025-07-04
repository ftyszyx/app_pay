use crate::entities::product;
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_products(
    State(db_pool): State<DatabaseConnection>,
) -> Result<Json<Vec<product::Model>>, StatusCode> {
    let products = product::Entity::find()
        .all(&db_pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}
