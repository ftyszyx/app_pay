use crate::constants;
use crate::types::common::{*};
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use crate::types::user_types::*;
use crate::handlers::user_handler;
use crate::utils::jwt::create_jwt;
use salvo::{prelude::*, oapi::extract::JsonBody};
use bcrypt::{verify};
use entity::roles;
use entity::users;
use sea_orm::{ ColumnTrait, EntityTrait,  QueryFilter};
use tracing::info;

#[handler]
pub async fn register(
    json: JsonBody<AuthPayload>,
    depot:&mut Depot,
) -> Result<ApiResponse<AuthResponse>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let user_exists = users::Entity::find()
        .filter(users::Column::Username.eq(&json.username))
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
        username: json.username.clone(),
        password: json.password.clone(),
        role_id: Some(user_role.id),
    }).await?;
    info!("User registered: {}", new_user.username);
    let token = create_jwt(new_user.id, user_role.name, &state.config.jwt)
        .map_err(|_| AppError::auth_failed("Token creation failed"))?;
    Ok(ApiResponse::success(AuthResponse { token }))
}

#[handler]
pub async fn login(
    payload:JsonBody<AuthPayload>,
    depot:&mut Depot,
) -> Result<ApiResponse<AuthResponse>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let user_result = users::Entity::find()
        .filter(users::Column::Username.eq(&payload.username.clone()))
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

#[handler]
pub async fn get_current_user(
    depot:&mut Depot,
) -> Result<ApiResponse<UserInfo>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let claims = depot.obtain::<Claims>().unwrap();
    let user_info=user_handler::get_by_id_impl(&state,claims.sub).await?;
    Ok(ApiResponse::success(user_info))
}

#[handler]
pub async fn change_password(
    payload: JsonBody<ChangePasswordPayload>,
    depot:&mut Depot,
) -> Result<ApiResponse<bool>, AppError> {
    use sea_orm::{EntityTrait, IntoActiveModel, ActiveModelTrait, Set};
    use entity::users;
    let state = depot.obtain::<AppState>().unwrap();
    let claims = depot.obtain::<Claims>().unwrap();
    let user = users::Entity::find_by_id(claims.sub).one(&state.db).await?;
    let user = user.ok_or(AppError::auth_failed("User not found"))?;
    verify(&payload.old_password, &user.password)
        .map_err(|_| AppError::auth_failed("Old password incorrect"))?;
    let mut active = user.into_active_model();
    active.password = Set(bcrypt::hash(payload.new_password.clone(), 10)?);
    let _ = active.update(&state.db).await?;
    Ok(ApiResponse::success(true))
}
