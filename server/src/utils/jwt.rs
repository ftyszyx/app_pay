use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Duration, Utc};
use crate::types::{common::Claims, error::AppError, config::JwtConfig};

pub fn create_jwt(user_id: i32, role: String, jwt_config: &JwtConfig) -> Result<String, AppError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(jwt_config.expire_days as i64))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user_id,
        role,
        exp: expiration as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_config.secret.as_ref()),
    )
    .map_err(|e| AppError::Message(format!("JWT encoding failed: {}", e)))
}
