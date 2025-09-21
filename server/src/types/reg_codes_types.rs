use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::types::common::ListParamsReq;

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate,Default)]
pub struct CreateRegCodeReq {
    pub code: String,
    pub app_id: i32,
    pub bind_device_info: Option<serde_json::Value>,
    pub valid_days: i32,
    pub max_devices: i32,
    pub status: i16,
    pub code_type: i16,
    pub expire_time: Option<DateTime<Utc>>,
    pub total_count: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate, IntoParams)]
pub struct RegCodeValidateReq {
    pub code: String,
    pub app_key: String,
    pub device_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct RegCodeValidateResp {
    pub code_type: i16,
    pub expire_time: Option<DateTime<Utc>>,
    pub remaining_count: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct UpdateRegCodeReq {
    pub code: Option<String>,
    pub app_id: Option<i32>,
    pub bind_device_info: Option<serde_json::Value>,
    pub valid_days: Option<i32>,
    pub max_devices: Option<i32>,
    pub status: Option<i16>,
    pub binding_time: Option<DateTime<Utc>>,
    pub code_type: Option<i16>,
    pub expire_time: Option<DateTime<Utc>>,
    pub total_count: Option<i32>,
    pub use_count: Option<i32>,
    pub device_id: Option<String>,
}

#[derive(Deserialize, ToSchema, Debug, Default, IntoParams)]
pub struct SearchRegCodesParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub app_id: Option<i32>,
    #[serde(default)]
    pub status: Option<i16>,
    #[serde(default)]
    pub code_type: Option<i16>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct RegCodeInfo {
    pub id: i32,
    pub code: String,
    pub app_id: i32,
    pub bind_device_info: Option<serde_json::Value>,
    pub valid_days: i32,
    pub max_devices: i32,
    pub status: i16,
    pub binding_time: Option<DateTime<Utc>>,
    pub code_type: i16,
    pub expire_time: Option<DateTime<Utc>>,
    pub total_count: Option<i32>,
    pub use_count: i32,
    pub device_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub app_name: Option<String>,
}

impl TryFrom<(entity::reg_codes::Model, Option<entity::apps::Model>)> for RegCodeInfo {
    type Error = crate::types::error::AppError;

    fn try_from(
        value: (entity::reg_codes::Model, Option<entity::apps::Model>)
    ) -> Result<Self, Self::Error> {
        let (reg_code, app) = value;
        Ok(Self {
            id: reg_code.id,
            code: reg_code.code,
            app_id: reg_code.app_id,
            bind_device_info: reg_code.bind_device_info,
            valid_days: reg_code.valid_days,
            max_devices: reg_code.max_devices,
            status: reg_code.status,
            binding_time: reg_code.binding_time,
            code_type: reg_code.code_type,
            expire_time: reg_code.expire_time,
            total_count: reg_code.total_count,
            use_count: reg_code.use_count,
            device_id: reg_code.device_id,
            created_at: reg_code.created_at,
            updated_at: reg_code.updated_at,
            app_name: app.map(|a| a.name),
        })
    }
}

impl TryFrom<entity::reg_codes::Model> for RegCodeInfo {
    type Error = crate::types::error::AppError;

    fn try_from(reg_code: entity::reg_codes::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: reg_code.id,
            code: reg_code.code,
            app_id: reg_code.app_id,
            bind_device_info: reg_code.bind_device_info,
            valid_days: reg_code.valid_days,
            max_devices: reg_code.max_devices,
            status: reg_code.status,
            binding_time: reg_code.binding_time,
            code_type: reg_code.code_type,
            expire_time: reg_code.expire_time,
            total_count: reg_code.total_count,
            use_count: reg_code.use_count,
            device_id: reg_code.device_id,
            created_at: reg_code.created_at,
            updated_at: reg_code.updated_at,
            app_name: None,
        })
    }
} 