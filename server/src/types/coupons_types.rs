use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::types::common::ListParamsReq;
use crate::utils::convert::from_str_optional;

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct CreateCouponReq {
    pub code: String,
    pub name: String,
    pub status: i16,
    pub discount_type: i16,
    pub discount_value: i64,
    pub min_purchase_amount: i64,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub usage_limit: i32,
    pub scope_type: i16,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct UpdateCouponReq {
    pub code: Option<String>,
    pub name: Option<String>,
    pub status: Option<i16>,
    pub discount_type: Option<i16>,
    pub discount_value: Option<i64>,
    pub min_purchase_amount: Option<i64>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub usage_limit: Option<i32>,
    pub scope_type: Option<i16>,
}

#[derive(Deserialize, ToSchema, Debug, Default, IntoParams)]
pub struct SearchCouponsParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(deserialize_with = "from_str_optional", default)]
    pub id: Option<i32>,
    pub code: Option<String>,
    pub name: Option<String>,
    #[serde(deserialize_with = "from_str_optional", default)]
    pub status: Option<i16>,
    #[serde(deserialize_with = "from_str_optional", default)]
    pub discount_type: Option<i16>,
    #[serde(deserialize_with = "from_str_optional", default)]
    pub scope_type: Option<i16>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct CouponInfo {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub status: i16,
    pub discount_type: i16,
    pub discount_value: i64,
    pub min_purchase_amount: i64,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub usage_limit: i32,
    pub scope_type: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<entity::coupons::Model> for CouponInfo {
    fn from(model: entity::coupons::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            status: model.status,
            discount_type: model.discount_type,
            discount_value: model.discount_value,
            min_purchase_amount: model.min_purchase_amount,
            start_time: model.start_time,
            end_time: model.end_time,
            usage_limit: model.usage_limit,
            scope_type: model.scope_type,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}