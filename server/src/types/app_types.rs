use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use crate::utils::convert::from_str_optional;

use crate::types::common::ListParamsReq;

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct AddAppReq {
    pub name: String,
    pub app_id: String,
    pub app_vername: String,
    pub app_vercode: i32,
    pub app_download_url: String,
    pub app_res_url: String,
    pub app_update_info: Option<String>,
    pub sort_order: i32,
    pub status: i16,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct UpdateAppReq {
    pub name: Option<String>,
    pub app_id: Option<String>,
    pub app_vername: Option<String>,
    pub app_vercode: Option<i32>,
    pub app_download_url: Option<String>,
    pub app_res_url: Option<String>,
    pub app_update_info: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<i16>,
}

#[derive(Serialize, ToSchema)]
pub struct AppListResponse {
    pub list: Vec<entity::apps::Model>,
    pub total: u64,
}

#[derive(Deserialize, ToSchema, Debug, Default, IntoParams)]
pub struct ListAppsParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(deserialize_with = "from_str_optional",default)]
    pub id: Option<i32>,
    pub app_id: Option<String>,
    pub name: Option<String>,
}
