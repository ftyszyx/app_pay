use super::config::Config;
use crate::utils::redis_cache::RedisCache;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use crate::utils::convert::from_str_optional;

#[derive(Deserialize, ToSchema, Debug,Serialize)]
pub struct ListParamsReq {
    #[serde(deserialize_with = "from_str_optional",default)]
    pub page: Option<u64>,
    #[serde(deserialize_with = "from_str_optional",default)]
    pub page_size: Option<u64>,
}

impl Default for ListParamsReq {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
        }
    }
}

#[derive(Debug, ToSchema, Serialize)]
pub struct PagingResponse<T> {
    pub list: Vec<T>,
    pub page: u64,
    pub total: u64,
}

// 创建一个应用状态结构体来管理所有共享状态
#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub redis: Arc<RedisCache>,
    pub config: Arc<Config>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: usize,
}
