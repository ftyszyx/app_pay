use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::types::common::ListParamsReq;

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct CreateOrderReq {
    pub order_id: String,
    pub user_info: Option<serde_json::Value>,
    pub status: i16,
    pub pay_method_id: i32,
    pub original_price: i64,
    pub final_price: i64,
    pub remark: Option<String>,
    pub created_by: i32,
    pub updated_by: i32,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct UpdateOrderReq {
    pub order_id: Option<String>,
    pub user_info: Option<serde_json::Value>,
    pub status: Option<i16>,
    pub pay_method_id: Option<i32>,
    pub original_price: Option<i64>,
    pub final_price: Option<i64>,
    pub remark: Option<String>,
    pub updated_by: Option<i32>,
}

#[derive(Deserialize, ToSchema, Debug, Default, IntoParams)]
pub struct SearchOrdersParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(default)]
    pub order_id: Option<String>,
    #[serde(default)]
    pub status: Option<i16>,
    #[serde(default)]
    pub pay_method_id: Option<i32>,
    #[serde(default)]
    pub created_by: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct OrderInfo {
    pub id: i32,
    pub order_id: String,
    pub user_info: Option<serde_json::Value>,
    pub status: i16,
    pub pay_method_id: i32,
    pub original_price: i64,
    pub final_price: i64,
    pub remark: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: i32,
    pub updated_by: i32,
    pub pay_method_name: Option<String>,
    pub created_by_username: Option<String>,
    pub updated_by_username: Option<String>,
}

impl TryFrom<(entity::orders::Model, Option<entity::pay_methods::Model>, Option<entity::users::Model>, Option<entity::users::Model>)> for OrderInfo {
    type Error = crate::types::error::AppError;

    fn try_from(
        value: (entity::orders::Model, Option<entity::pay_methods::Model>, Option<entity::users::Model>, Option<entity::users::Model>)
    ) -> Result<Self, Self::Error> {
        let (order, pay_method, created_by_user, updated_by_user) = value;
        Ok(Self {
            id: order.id,
            order_id: order.order_id,
            user_info: order.user_info,
            status: order.status,
            pay_method_id: order.pay_method_id,
            original_price: order.original_price,
            final_price: order.final_price,
            remark: order.remark,
            created_at: order.created_at,
            updated_at: order.updated_at,
            created_by: order.created_by,
            updated_by: order.updated_by,
            pay_method_name: pay_method.map(|pm| pm.name),
            created_by_username: created_by_user.map(|u| u.username),
            updated_by_username: updated_by_user.map(|u| u.username),
        })
    }
}

impl TryFrom<entity::orders::Model> for OrderInfo {
    type Error = crate::types::error::AppError;

    fn try_from(order: entity::orders::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: order.id,
            order_id: order.order_id,
            user_info: order.user_info,
            status: order.status,
            pay_method_id: order.pay_method_id,
            original_price: order.original_price,
            final_price: order.final_price,
            remark: order.remark,
            created_at: order.created_at,
            updated_at: order.updated_at,
            created_by: order.created_by,
            updated_by: order.updated_by,
            pay_method_name: None,
            created_by_username: None,
            updated_by_username: None,
        })
    }
} 