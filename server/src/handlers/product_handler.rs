use axum::{extract::State, Json};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entities::products;

pub async fn get_all_products(
    State(db): State<DatabaseConnection>,
) -> Json<Vec<products::Model>> {
    let products = products::Entity::find().all(&db).await.unwrap();
    Json(products)
} 