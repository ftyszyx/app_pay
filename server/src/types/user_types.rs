use entity::{roles, users};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use crate::types::{common::ListParamsReq, error::AppError};
use crate::utils::convert::from_str_optional;

#[derive(Deserialize, ToSchema)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub balance: i64,
    pub invite_count: u64,
    pub invite_rebate_total: i64,
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct UserCreatePayload {
    pub username: String,
    pub password: String,
    pub role_id: Option<i32>,
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct UserUpdatePayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role_id: Option<i32>,
    pub balance: Option<i64>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub balance: String,
    pub inviter_id: Option<i32>,
    pub invite_count: u64,
    pub invite_rebate_total: i64,
    pub role_id: i32,
    pub role_name: String,
    pub created_at: String,
}

impl TryFrom<(users::Model, Option<roles::Model>)> for UserInfo {
    type Error = AppError;
    fn try_from((u, r): (users::Model, Option<roles::Model>)) -> Result<Self, Self::Error> {
        let role = r.ok_or_else(|| AppError::Message("role not found".to_string()))?;
        Ok(Self {
            id: u.id,
            username: u.username,
            balance: u.balance.to_string(),
            inviter_id: u.inviter_id,
            invite_count: u.invite_count,
            invite_rebate_total: 0,
            role_id: u.role_id,
            role_name: role.name,
            created_at: u.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }
}

#[derive(ToSchema, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize, ToSchema, Debug, Default, IntoParams)]
pub struct SearchUsersParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    pub username: Option<String>,
    pub user_id: Option<String>,
    #[serde(deserialize_with = "from_str_optional",default)]
    pub id: Option<i32>,
}
