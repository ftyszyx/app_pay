use axum::{response::{IntoResponse, Response}};
use crate::types::response::ApiResponse;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DbErr),
    DataNotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            AppError::DataNotFound => write!(f, "App not found"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        ApiResponse::<()>::error_with_message(self.to_string()).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DatabaseError(err)
    }
}

#[derive(Deserialize)]
pub struct ListParamsReq {
    pub page: u64,
    pub page_size: u64,
}

#[derive(Debug, ToSchema, Serialize)]
pub struct PagingResponse<T> {
    pub list: Vec<T>,
    pub page: u64,
    pub total: u64,
}

