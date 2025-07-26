use axum::response::IntoResponse;
use sea_orm::DbErr;
use std::{convert::Infallible, fmt};

use crate::types::response::ApiResponse;

/// 改进的错误处理系统
#[derive(Debug)]
#[allow(dead_code)]
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
    NotImplemented {
        message: String,
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
    UserAlreadyExists,
    Message(String),
}

#[allow(dead_code)]
impl AppError {
    /// 创建验证错误的便捷方法
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
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

    pub fn user_already_exists() -> Self {
        Self::UserAlreadyExists
    }

    /// 创建业务逻辑错误的便捷方法
    pub fn business_logic(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::BusinessLogic {
            code: code.into(),
            message: message.into(),
        }
    }

    /// 获取错误代码
    pub fn error_code(&self) -> u16 {
        match self {
            Self::Database(_) => crate::constants::APP_OTHER,
            Self::NotFound { .. } => crate::constants::APP_NOT_FOUND,
            Self::Validation { .. } => crate::constants::APP_VALIDATION_ERROR,
            Self::AuthFailed { .. } => crate::constants::APP_AUTH_FAILED,
            Self::Forbidden { .. } => crate::constants::APP_FORBIDDEN,
            Self::BusinessLogic { .. } => crate::constants::APP_BUSINESS_LOGIC,
            Self::ExternalService { .. } => crate::constants::APP_EXTERNAL_SERVICE,
            Self::UserAlreadyExists => crate::constants::APP_USER_ALREADY_EXISTS,
            Self::NotImplemented { .. } => crate::constants::APP_NOT_IMPLEMENTED,
            Self::Message(_) => crate::constants::APP_OTHER,
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
            Self::Validation { message } => {
                write!(f, "Validation error: {}", message)
            }
            Self::AuthFailed { reason } => write!(f, "Authentication failed: {}", reason),
            Self::Forbidden { action } => write!(f, "Forbidden action: {}", action),
            Self::BusinessLogic { code, message } => {
                write!(f, "Business logic error [{}]: {}", code, message)
            }
            Self::ExternalService { service, error } => {
                write!(f, "External service '{}' error: {}", service, error)
            }
            Self::UserAlreadyExists => write!(f, "User already exists"),
            Self::NotImplemented { message } => write!(f, "Not implemented: {}", message),
            Self::Message(message) => write!(f, "{}", message),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_code = self.error_code();
        let message = self.to_string();
        let response = ApiResponse::<()> {
            code: error_code,
            message,
            data: None,
            success: false,
        };
        return response.into_response();
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        tracing::error!("Validation errors: {:?}", errors);
        Self::Validation {
            message: errors.to_string(),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        tracing::error!("Database error: {}", err);
        Self::Database(err)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        tracing::error!("Password hashing error: {}", err);
        Self::BusinessLogic {
            code: "PASSWORD_HASH_ERROR".to_string(),
            message: "Failed to hash password".to_string(),
        }
    }
}

impl From<Infallible> for AppError {
    fn from(err: Infallible) -> Self {
        tracing::error!("Infallible error: {:?}", err);
        Self::Message("Infallible error".to_string())
    }
}
