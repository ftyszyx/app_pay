use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(DbErr),
    UserNotFound,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(e) => write!(f, "Database Error: {}", e),
            AppError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DatabaseError(err)
    }
}