use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use crate::utils::convert::from_str_optional;

use crate::types::common::ListParamsReq;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct ProductCreatePayload {
    pub name: String,
    pub price: i32,
    pub app_id: i32,
    pub product_id: String,
    pub add_valid_days: i32,
    pub image_url: Option<String>,
    pub tags: Option<String>,
    pub status: i16,
    pub remark: Option<String>,
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct ProductUpdatePayload {
    pub name: Option<String>,
    pub price: Option<i32>,
    pub app_id: Option<i32>,
    pub product_id: Option<String>,
    pub add_valid_days: Option<i32>,
    pub image_url: Option<String>,
    pub tags: Option<String>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Serialize, ToSchema, Debug)]
pub struct ProductListResponse {
    pub list: Vec<entity::products::Model>,
    pub total: u64,
}

#[derive(Deserialize, ToSchema, Debug, Default, IntoParams)]
pub struct ListProductsParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(deserialize_with = "from_str_optional",default)]
    pub id: Option<i32>,
    pub product_id: Option<String>,
    pub name: Option<String>,
}
