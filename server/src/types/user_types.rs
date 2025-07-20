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


#[derive(Serialize, ToSchema)]
pub struct UserListResponse {
    pub list: Vec<UserInfo>,
    pub total: u64,
}

#[derive(ToSchema, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
} 

#[derive(Deserialize, ToSchema)]
pub struct ListUsersParams{
    pub username: Option<String>,
}