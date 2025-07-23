use crate::utils::redis_cache::RedisCache;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;
use super::config::Config;

#[derive(Deserialize, ToSchema,Debug)]
pub struct ListParamsReq{
    pub page: u64,
    pub page_size: u64,
}

impl Default for ListParamsReq {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
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