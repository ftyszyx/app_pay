use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;
use crate::my_error::ErrorCode;

/// The data part of the API response.
#[derive(Serialize, ToSchema)]
pub struct ResponseBody<T> {
    #[schema(example = 200)]
    pub code: u16,

    #[schema(example = "Success")]
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// A unified response structure for all APIs.
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T> {
    #[schema(example = 200)]
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ResponseBody<T>>,
}

impl<T> ApiResponse<T> {
    /// Creates a new successful response with data.
    pub fn success(data: T) -> Self {
        Self {
            status: 200,
            data: Some(ResponseBody {
                code: ErrorCode::Ok.details().1,
                message: ErrorCode::Ok.details().0.to_string(),
                data: Some(data),
            }),
        }
    }

    pub  fn error_with_message(message:String)->Self{
        Self {
            status: StatusCode::OK.as_u16(),
            data: Some(ResponseBody {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message,
                data: None,
            }),
        }
    }

    pub fn error_with_code(error: ErrorCode)->Self{
        Self {
            status: StatusCode::OK.as_u16(),
            data: Some(ResponseBody {
                code: error.details().1,
                message: error.details().0.to_string(),
                data: None,
            }),
        }
    }

    /// Creates a new error response.
    /// The data field will be None.
    pub fn error(status_code: StatusCode, message: String) -> Self {
        Self {
            status: status_code.as_u16(),
            data: Some(ResponseBody {
                code: status_code.as_u16(),
                message,
                data: None,
            }),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let status_code = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let mut response = Json(self).into_response();
        *response.status_mut() = status_code;
        response
    }
} 