use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ProductCreatePayload {
    pub name: String,
    pub price: i32,
    pub app_id: i32,
    pub product_id: String,
    pub add_valid_days: i32,
    pub image_url: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub status: i16,
    pub remark: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ProductUpdatePayload {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub app_id: Option<i32>,
    pub product_id: Option<String>,
    pub add_valid_days: Option<i32>,
    pub image_url: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ProductListResponse {
    pub list: Vec<entity::products::Model>,
    pub total: u64,
}

#[derive(Deserialize, ToSchema)]
pub struct ListProductsParams {
    pub name: Option<String>,
    pub app_id: Option<i32>,
} 