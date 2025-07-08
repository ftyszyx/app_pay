use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::env;

use crate::entities::user::{self, Entity as User};

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    if User::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .is_some()
    {
        return Err(StatusCode::CONFLICT);
    }

    let hashed_password =
        hash(&payload.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = user::ActiveModel {
        username: Set(payload.username),
        password: Set(hashed_password),
        ..Default::default()
    };

    let saved_user = new_user
        .insert(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let token = create_jwt(saved_user.id)?;
    Ok(Json(AuthResponse { token }))
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = User::find()
        .filter(user::Column::Username.eq(&payload.username))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !verify(&payload.password, &user.password).unwrap_or(false) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_jwt(user.id)?;
    Ok(Json(AuthResponse { token }))
}

fn create_jwt(user_id: i32) -> Result<String, StatusCode> {
    let secret = env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
