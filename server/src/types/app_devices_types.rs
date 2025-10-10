use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use salvo_oapi::ToSchema;
use crate::types::common::ListParamsReq;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DeviceInfo {
    pub id: i32,
    pub app_id: i32,
    pub app_name: String,
    pub device_id: String,
    pub device_info: Option<serde_json::Value>,
    pub bind_time: Option<DateTime<Utc>>,
    pub expire_time: Option<DateTime<Utc>>,
}

impl TryFrom<(entity::app_devices::Model,Option<entity::apps::Model>)> for DeviceInfo {
    type Error = crate::types::error::AppError;
    fn try_from(value: (entity::app_devices::Model,Option<entity::apps::Model>)) -> Result<Self, Self::Error> {
        let (app_device, app) = value;
        Ok(Self {
            id: app_device.id,
            app_id: app_device.app_id,
            app_name: app.map(|a| a.name).unwrap_or_default(),
            device_id: app_device.device_id,
            device_info: app_device.device_info,
            bind_time: app_device.bind_time,
            expire_time: app_device.expire_time,
        })
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct SearchDevicesParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    pub app_id: Option<i32>,
    pub device_id: Option<String>,
}