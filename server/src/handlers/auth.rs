use crate::types::common::AppState;
use crate::types::{common::Claims, error::AppError, response::ApiResponse};
use crate::types::user_types::{AuthPayload, AuthResponse, UserResponse};
use crate::utils::jwt::create_jwt;
use crate::{constants};
use axum::Extension;
use axum::{Json, extract::State};
use bcrypt::{DEFAULT_COST, hash, verify};
use entity::invite_records;
use entity::roles;
use entity::users;
use sea_orm::{ ActiveModelTrait, ColumnTrait,  EntityTrait, PaginatorTrait, QueryFilter, Set, };
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
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<ApiResponse<AuthResponse>, AppError> {
    let user_exists = users::Entity::find()
        .filter(users::Column::Username.eq(&payload.username))
        .one(&state.db)
        .await?;
    if user_exists.is_some() {
        return Err(AppError::Message("user already exists".to_string()));
    }

    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::auth_failed("Password hash failed"))?;
    let user_role = roles::Entity::find()
        .filter(roles::Column::Name.eq(constants::USER_ROLE))
        .one(&state.db)
        .await?;
    let user_role = user_role.ok_or(AppError::not_found("role", None))?;
    let new_user = users::ActiveModel {
        username: Set(payload.username),
        password: Set(hashed_password),
        role_id: Set(user_role.id),
        ..Default::default()
    };
    let saved_user = new_user.insert(&state.db).await?;

    info!("User registered: {}", saved_user.username);
    let token = create_jwt(saved_user.id, user_role.name)
        .map_err(|_| AppError::auth_failed("Token creation failed"))?;
    Ok(ApiResponse::success(AuthResponse { token }))
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
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Result<ApiResponse<AuthResponse>, AppError> {
    let user_result = users::Entity::find()
        .filter(users::Column::Username.eq(&payload.username))
        .find_also_related(roles::Entity)
        .find_also_related(roles::Entity)
        .one(&state.db)
        .await?;
    let user_result = user_result.ok_or(AppError::NotFound {
        resource: "user".to_string(),
        id: None,
    })?;
    verify(&payload.password, &user_result.0.password)
        .map_err(|_| AppError::auth_failed("User or password error"))?;
    let role_name = user_result
        .1
        .map_or(constants::USER_ROLE.to_string(), |r| r.name);
    info!("User logged in: {}", user_result.0.username);
    let token = create_jwt(user_result.0.id, role_name)
        .map_err(|_| AppError::auth_failed("Token creation failed"))?;
    Ok(ApiResponse::success(AuthResponse { token }))
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
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<UserResponse>, AppError> {
    let user_with_role = users::Entity::find_by_id(claims.sub)
        .find_also_related(roles::Entity)
        .one(&state.db)
        .await?;
    let user_with_role = user_with_role.ok_or(AppError::NotFound {
        resource: "user".to_string(),
        id: Some(claims.sub),
    })?;
    //get invite count from invite records
    let invite_count = invite_records::Entity::find()
        .filter(invite_records::Column::InviterId.eq(claims.sub))
        .count(&state.db)
        .await
        .unwrap_or(0);

    let role_name = user_with_role.1.map_or("user".to_string(), |r| r.name);
    let response = UserResponse {
        id: user_with_role.0.id,
        username: user_with_role.0.username,
        role: role_name,
        balance: user_with_role.0.balance,
        invite_count,
        invite_rebate_total: user_with_role.0.invite_rebate_total,
    };
    Ok(ApiResponse::success(response))
}

