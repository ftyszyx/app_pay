use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
/// 改进的错误处理系统
#[derive(Debug)]
pub enum AppError {
    /// 数据库相关错误
    Database(DbErr),
    /// 资源未找到错误
    NotFound {
        resource: String,
        id: Option<i32>,
    },
    /// 验证错误
    Validation {
        field: String,
        message: String,
    },

    /// 认证失败
    AuthFailed {
        reason: String,
    },

    /// 权限不足
    Forbidden {
        action: String,
    },

    /// 业务逻辑错误
    BusinessLogic {
        code: String,
        message: String,
    },

    /// 外部服务错误
    ExternalService {
        service: String,
        error: String,
    },

    // 保持向后兼容
    DataNotFound,
    Other(String),
}

impl AppError {
    /// 创建验证错误的便捷方法
    pub fn validation(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Validation {
            field: field.into(),
            message: message.into(),
        }
    }

    /// 创建未找到错误的便捷方法
    pub fn not_found(resource: impl Into<String>, id: Option<i32>) -> Self {
        Self::NotFound {
            resource: resource.into(),
            id,
        }
    }

    /// 创建认证错误的便捷方法
    pub fn auth_failed(reason: impl Into<String>) -> Self {
        Self::AuthFailed {
            reason: reason.into(),
        }
    }

    /// 创建业务逻辑错误的便捷方法
    pub fn business_logic(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::BusinessLogic {
            code: code.into(),
            message: message.into(),
        }
    }

    /// 获取HTTP状态码
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound { .. } | Self::DataNotFound => StatusCode::NOT_FOUND,
            Self::Validation { .. } => StatusCode::BAD_REQUEST,
            Self::AuthFailed { .. } => StatusCode::UNAUTHORIZED,
            Self::Forbidden { .. } => StatusCode::FORBIDDEN,
            Self::BusinessLogic { .. } => StatusCode::BAD_REQUEST,
            Self::ExternalService { .. } => StatusCode::BAD_GATEWAY,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// 获取错误代码
    pub fn error_code(&self) -> u16 {
        match self {
            Self::Database(_) => 5001,
            Self::NotFound { .. } | Self::DataNotFound => 4004,
            Self::Validation { .. } => 4001,
            Self::AuthFailed { .. } => 4010,
            Self::Forbidden { .. } => 4030,
            Self::BusinessLogic { .. } => 4002,
            Self::ExternalService { .. } => 5020,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::NotFound { resource, id } => match id {
                Some(id) => write!(f, "Resource '{}' with id {} not found", resource, id),
                None => write!(f, "Resource '{}' not found", resource),
            },
            Self::Validation { field, message } => {
                write!(f, "Validation error for field '{}': {}", field, message)
            }
            Self::AuthFailed { reason } => write!(f, "Authentication failed: {}", reason),
            Self::Forbidden { action } => write!(f, "Forbidden action: {}", action),
            Self::BusinessLogic { code, message } => {
                write!(f, "Business logic error [{}]: {}", code, message)
            }
            Self::ExternalService { service, error } => {
                write!(f, "External service '{}' error: {}", service, error)
            }
            Self::DataNotFound => write!(f, "Data not found"),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let error_code = self.error_code();
        let message = self.to_string();

        let response = ApiResponse::<()> {
            code: error_code,
            message,
            data: None,
            success: false,
        };

        (status_code, axum::Json(response)).into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        tracing::error!("Validation errors: {:?}", errors);
        Self::Validation {
            field: "".to_string(),
            message: "Validation failed".to_string(),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        tracing::error!("Database error: {}", err);
        Self::Database(err)
    }
}

// 为bcrypt错误添加转换
impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        tracing::error!("Password hashing error: {}", err);
        Self::BusinessLogic {
            code: "PASSWORD_HASH_ERROR".to_string(),
            message: "Failed to hash password".to_string(),
        }
    }
}
