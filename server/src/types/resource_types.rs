use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::convert::from_str_optional;
use crate::types::common::ListParamsReq;

#[derive(Deserialize, Debug, Validate)]
pub struct ResourceCreatePayload {
    pub name: String,
    pub object_key: String,
    pub url: String,
    pub path: String,
    pub file_type: String,
    pub tags: Option<Vec<String>>,
    pub status: i16,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, Validate, Default)]
pub struct ResourceUpdatePayload {
    pub name: Option<String>,
    pub object_key: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
    pub file_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub status: Option<i16>,
    pub remark: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ResourceListResponse {
    pub list: Vec<entity::resources::Model>,
    pub total: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListResourcesParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(deserialize_with = "from_str_optional", default)]
    pub id: Option<i32>,
    pub name: Option<String>,
    pub object_key: Option<String>,
    pub url: Option<String>,
    pub path: Option<String>,
    #[serde(deserialize_with = "from_str_optional", default)]
    pub status: Option<i16>,
}

