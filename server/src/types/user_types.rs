use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use validator::Validate;
use crate::types::common::ListParamsReq;
use crate::utils::convert::from_str_optional;

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UserCreatePayload {
    pub username: String,
    pub password: String,
    pub role_id: Option<i32>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct UserUpdatePayload {
    pub username: Option<String>,
    pub password: Option<String>,
    pub role_id: Option<i32>,
    pub balance: Option<i64>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct ChangePasswordPayload {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Deserialize, Serialize, Debug,FromQueryResult)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub balance: i64,
    pub inviter_id: Option<i32>,
    pub inviter_username: Option<String>,
    pub invite_count: i64,
    pub invite_rebate_total: i64,
    pub role_id: i32,
    pub role_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct SearchUsersParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    pub username: Option<String>,
    #[serde(deserialize_with = "from_str_optional",default)]
    pub id: Option<i32>,
}
