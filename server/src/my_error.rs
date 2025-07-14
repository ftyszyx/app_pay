//! Defines custom error codes for the application.
pub enum ErrorCode {
    Ok,
    UserAlreadyExists,
    UserNotFound,
    UserOrPasswordError,
    TokenCreationFailed,
    DefaultRoleNotFound,
    DatabaseError,
}

impl ErrorCode {
    pub fn details(&self) -> (&'static str, u16) {
        match self {
            Self::Ok => ("Ok", 0),
            Self::UserAlreadyExists => ("User already exists", 1001),
            Self::UserNotFound => ("User not found", 1002),
            Self::UserOrPasswordError => ("User or password error", 1003),
            Self::TokenCreationFailed => ("Token creation failed", 1004),
            Self::DefaultRoleNotFound => ("Default role not found", 1005),
            Self::DatabaseError => ("Database error", 5001),
        }
    }
}