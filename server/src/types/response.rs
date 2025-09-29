use salvo::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
    pub success: bool,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: crate::constants::APP_OK,
            message: "success".to_string(),
            data: Some(data),
            success: true,
        }
    }

    #[allow(dead_code)]
    pub fn error_with_message(message: String) -> Self {
        Self {
            code: crate::constants::APP_OTHER,
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

