use sea_orm::DbErr;
use serde::Deserialize;
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DbErr),
    UserNotFound,
    AppNotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            AppError::UserNotFound => write!(f, "User not found"),
            AppError::AppNotFound => write!(f, "App not found"),
        }
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

#[derive(Debug, ToSchema)]
pub struct PagingResponse<T> {
    pub list: Vec<T>,
    pub page: u64,
    pub total: u64,
}
