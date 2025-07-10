use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::info;
use utoipa::ToSchema;

use crate::entities::{role, user};

#[derive(Deserialize, ToSchema)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: i32,
    role: String,
    exp: usize,
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/register",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "User created successfully", body = AuthResponse),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    if user::Entity::find()
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

    let user_role = role::Entity::find()
        .filter(role::Column::Name.eq("user"))
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let new_user = user::ActiveModel {
        username: Set(payload.username),
        password: Set(hashed_password),
        role_id: Set(Some(user_role.id)),
        ..Default::default()
    };

    let saved_user = new_user
        .insert(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    info!("User registered: {}", saved_user.username);
    let token = create_jwt(saved_user.id, "user".to_string())?;
    Ok(Json(AuthResponse { token }))
}

/// Login as an existing user
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthResponse>, StatusCode> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(&payload.username))
        .find_also_related(role::Entity)
        .one(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let (user_model, role_model) = user;

    if !verify(&payload.password, &user_model.password).unwrap_or(false) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let role_name = role_model.map_or("user".to_string(), |r| r.name);

    info!("User logged in: {}", user_model.username);
    let token = create_jwt(user_model.id, role_name)?;
    Ok(Json(AuthResponse { token }))
}

fn create_jwt(user_id: i32, role: String) -> Result<String, StatusCode> {
    let secret = env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
