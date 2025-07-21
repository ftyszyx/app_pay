use crate::types::common::AppError;
use entity::{invite_records, roles, users};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

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
    pub invite_count: i32,
    pub invite_rebate_total: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct UserCreatePayload {
    pub username: String,
    pub password: String,
    pub role_id: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub struct UserUpdatePayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role_id: Option<i32>,
    pub balance: Option<i64>,
}

#[derive(FromRow, Serialize, ToSchema, Debug)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub balance: String,
    pub inviter_id: Option<i32>,
    pub invite_count: i32,
    pub invite_rebate_total: i64,
    pub role_id: i32,
    pub role_name: String,
    pub created_at: String,
}

pub async fn model_to_info(u: users::Model, db: &DatabaseConnection) -> Result<UserInfo, AppError> {
    let (role_id, role_name) = {
        match roles::Entity::find_by_id(u.role_id).one(db).await {
            Ok(Some(role)) => (role.id, role.name),
            Ok(None) => return Err(AppError::DataNotFound),
            Err(e) => return Err(e.into()),
        }
    };

    let invite_count = invite_records::Entity::find()
        .filter(invite_records::Column::InviterId.eq(u.id))
        .count(db)
        .await? as i32;
    let balance = u.balance.to_string();
    let invite_rebate_total = u.invite_rebate_total;
    let created_at = u.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(UserInfo {
        id: u.id,
        username: u.username,
        balance,
        inviter_id: u.inviter_id,
        invite_count,
        invite_rebate_total,
        role_id,
        role_name,
        created_at,
    })
}

#[derive(ToSchema, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ListUsersParams {
    pub username: Option<String>,
}
