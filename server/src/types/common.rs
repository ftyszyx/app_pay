use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize)]
pub struct ListParamsReq {
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
