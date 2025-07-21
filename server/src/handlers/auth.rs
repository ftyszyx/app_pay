use crate::handlers::middleware::Claims;
use crate::types::response::ApiResponse;
use crate::types::user_types::{AuthPayload, AuthResponse, UserResponse};
use crate::{constants, my_error};
use axum::Extension;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use entity::invite_records;
use entity::roles;
use entity::users;
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use std::env;
use tracing::info;

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/register",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "User created successfully", body = ApiResponse<AuthResponse>),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    let user_exists = users::Entity::find()
        .filter(users::Column::Username.eq(&payload.username))
        .one(&db)
        .await;

    match user_exists {
        Ok(Some(_)) => {
            return ApiResponse::<AuthResponse>::error_with_code(
                my_error::ErrorCode::UserAlreadyExists,
            );
        }
        Err(error) => {
            tracing::error!("Error checking user existence: {:?}", error);
            return ApiResponse::<AuthResponse>::error_with_code(
                my_error::ErrorCode::DatabaseError,
            );
        }
        Ok(None) => {}
    }

    let hashed_password = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(msg) => {
            return ApiResponse::<AuthResponse>::error_with_message(msg.to_string());
        }
    };

    let user_role = match roles::Entity::find()
        .filter(roles::Column::Name.eq(constants::USER_ROLE))
        .one(&db)
        .await
    {
        Ok(Some(role)) => role,
        Ok(None) => {
            return ApiResponse::<AuthResponse>::error_with_code(
                my_error::ErrorCode::DefaultRoleNotFound,
            );
        }
        Err(err) => {
            tracing::error!("Error getting user role: {:?}", err);
            return ApiResponse::<AuthResponse>::error_with_code(
                my_error::ErrorCode::DatabaseError,
            );
        }
    };

    let new_user = users::ActiveModel {
        username: Set(payload.username),
        password: Set(hashed_password),
        role_id: Set(user_role.id),
        ..Default::default()
    };
    tracing::info!("new_user: {:?}", new_user);

    let saved_user = match new_user.insert(&db).await {
        Ok(user) => user,
        Err(err) => {
            tracing::error!("Error inserting user: {:?}", err);
            return ApiResponse::<AuthResponse>::error_with_code(
                my_error::ErrorCode::DatabaseError,
            );
        }
    };

    info!("User registered: {}", saved_user.username);
    match create_jwt(saved_user.id, constants::USER_ROLE.to_string()) {
        Ok(token) => ApiResponse::success(AuthResponse { token }),
        Err(err) => {
            tracing::error!("Error creating JWT: {:?}", err);
            ApiResponse::<AuthResponse>::error_with_code(my_error::ErrorCode::TokenCreationFailed)
        }
    }
}

/// Login as an existing user
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse<AuthResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    let user_result = users::Entity::find()
        .filter(users::Column::Username.eq(&payload.username))
        .find_also_related(roles::Entity)
        .one(&db)
        .await;

    let (user_model, role_model) = match user_result {
        Ok(Some(user)) => user,
        Ok(None) => {
            return ApiResponse::<AuthResponse>::error_with_code(my_error::ErrorCode::UserNotFound);
        }
        Err(err) => {
            tracing::error!("Error getting user: {:?}", err);
            return ApiResponse::<AuthResponse>::error_with_code(
                my_error::ErrorCode::DatabaseError,
            );
        }
    };

    if !verify(&payload.password, &user_model.password).unwrap_or(false) {
        return ApiResponse::<AuthResponse>::error_with_code(
            my_error::ErrorCode::UserOrPasswordError,
        );
    }

    let role_name = role_model.map_or(constants::USER_ROLE.to_string(), |r| r.name);

    info!("User logged in: {}", user_model.username);
    match create_jwt(user_model.id, role_name) {
        Ok(token) => ApiResponse::success(AuthResponse { token }),
        Err(err) => {
            tracing::error!("Error creating JWT: {:?}", err);
            ApiResponse::<AuthResponse>::error_with_code(my_error::ErrorCode::TokenCreationFailed)
        }
    }
}

/// Get current user information
#[utoipa::path(
    get,
    path = "/api/me",
    responses(
        (status = 200, description = "Current user information", body = ApiResponse<UserResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_current_user(
    State(db): State<DatabaseConnection>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let user_with_role = users::Entity::find_by_id(claims.sub)
        .find_also_related(roles::Entity)
        .one(&db)
        .await;
    //get invite count from invite records
    let invite_count = match invite_records::Entity::find()
        .filter(invite_records::Column::InviterId.eq(claims.sub))
        .count(&db)
        .await
    {
        Ok(count) => count as i32,
        Err(_) => {
            return ApiResponse::<UserResponse>::error_with_code(
                my_error::ErrorCode::DatabaseError,
            );
        }
    };

    match user_with_role {
        Ok(Some((user, role))) => {
            let role_name = role.map_or("user".to_string(), |r| r.name);
            let response = UserResponse {
                id: user.id,
                username: user.username,
                role: role_name,
                balance: user.balance,
                invite_count,
                invite_rebate_total: user.invite_rebate_total,
            };
            ApiResponse::success(response)
        }
        Ok(None) => ApiResponse::<UserResponse>::error_with_code(my_error::ErrorCode::UserNotFound),
        Err(_) => ApiResponse::<UserResponse>::error_with_code(my_error::ErrorCode::DatabaseError),
    }
}

fn create_jwt(user_id: i32, role: String) -> Result<String, StatusCode> {
    let secret = env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let expire: u32 = env::var("JWT_EXPIRE")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
