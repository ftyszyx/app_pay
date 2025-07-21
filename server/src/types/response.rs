use crate::my_error::ErrorCode;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
    pub success: bool,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        let (message, code) = ErrorCode::Ok.details();
        Self {
            code,
            message: message.to_string(),
            data: Some(data),
            success: true,
        }
    }

    pub fn error_with_code(error_code: ErrorCode) -> Self {
        let (message, code) = error_code.details();
        Self {
            code,
            message: message.to_string(),
            data: None,
            success: false,
        }
    }

    pub fn error_with_message(message: String) -> Self {
        let (_, code) = ErrorCode::DatabaseError.details();
        Self {
            code,
            message,
            data: None,
            success: false,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
