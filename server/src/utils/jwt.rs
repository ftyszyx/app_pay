use std::env;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Duration, Utc};

use crate::types::{common::Claims, error::AppError};


pub fn create_jwt(user_id: i32, role: String) -> Result<String, AppError> {
    let secret = env::var("JWT_SECRET").map_err(|_| AppError::Message("JWT_SECRET not found".to_string()))?;
    let expire: u32 = env::var("JWT_EXPIRE")
        .map_err(|_| AppError::Message("JWT_EXPIRE not found".to_string()))?
        .parse()
        .map_err(|_| AppError::Message("JWT_EXPIRE not found".to_string()))?;
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(expire as i64))
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
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| AppError::Message("JWT_SECRET not found".to_string()))
}
