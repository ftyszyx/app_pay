use entity::{invite_records, roles, users};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::types::error::AppError;

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

impl From<(users::Model,roles::Model)> for UserInfo {
    fn from((u,r): (users::Model,roles::Model)) -> Self {
        Self{
            id: u.id,
            username: u.username,
            balance: u.balance.to_string(),
            inviter_id: u.inviter_id,
            invite_count: 0,
            invite_rebate_total: 0,
            role_id: u.role_id,
            role_name: r.name,
            created_at: u.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}


#[derive(ToSchema, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct SearchUsersParams {
    pub username: Option<String>,
    pub id: Option<i32>,
}
