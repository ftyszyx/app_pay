use crate::constants;
use crate::handlers::user_handler::{self};
use crate::types::common::AppState;
use crate::types::user_types::{AuthPayload, AuthResponse, UserCreatePayload, UserInfo };
use crate::types::user_types::ChangePasswordPayload;
use crate::types::{common::Claims, error::AppError, response::ApiResponse};
use crate::utils::jwt::create_jwt;
use axum::Extension;
use axum::{Json, extract::State};
use bcrypt::{verify};
use bcrypt::hash;
use entity::roles;
use entity::users;
use sea_orm::{ ColumnTrait, EntityTrait,  QueryFilter};
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
        return Err(AppError::user_already_exists());
    }
    let user_role = roles::Entity::find()
        .filter(roles::Column::Name.eq(constants::USER_ROLE))
        .one(&state.db)
        .await?;
    let user_role = user_role.ok_or(AppError::not_found("role", None))?;
    let new_user = user_handler::add_impl(&state,UserCreatePayload {
        username: payload.username,
        password: payload.password,
        role_id: Some(user_role.id),
    }).await?;
    info!("User registered: {}", new_user.username);
    let token = create_jwt(new_user.id, user_role.name, &state.config.jwt)
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
    let token = create_jwt(user_result.0.id, role_name, &state.config.jwt)
        .map_err(|_| AppError::auth_failed("Token creation failed"))?;
    Ok(ApiResponse::success(AuthResponse { token }))
}

/// Get current user information
#[utoipa::path(
    get,
    path = "/api/me",
    responses(
        (status = 200, description = "Current user information", body = ApiResponse<UserInfo>),
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
) -> Result<ApiResponse<UserInfo>, AppError> {
    let user_info=user_handler::get_by_id_impl(&state,claims.sub).await?;
    Ok(ApiResponse::success(user_info))
}

/// Change current user's password
#[utoipa::path(
    post,
    path = "/api/admin/me/password",
    request_body = ChangePasswordPayload,
    responses(
        (status = 200, description = "Password changed", body = ApiResponse<bool>),
        (status = 401, description = "Unauthorized"),
        (status = 400, description = "Bad request"),
    ),
    security(("api_key" = []))
)]
pub async fn change_password(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePasswordPayload>,
) -> Result<ApiResponse<bool>, AppError> {
    use sea_orm::{EntityTrait, IntoActiveModel, ActiveModelTrait, Set};
    use entity::users;
    let user = users::Entity::find_by_id(claims.sub).one(&state.db).await?;
    let user = user.ok_or(AppError::auth_failed("User not found"))?;
    verify(&payload.old_password, &user.password)
        .map_err(|_| AppError::auth_failed("Old password incorrect"))?;
    let mut active = user.into_active_model();
    active.password = Set(hash(payload.new_password, 10)?);
    let _ = active.update(&state.db).await?;
    Ok(ApiResponse::success(true))
}
