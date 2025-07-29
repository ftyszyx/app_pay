use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use chrono::{DateTime, Utc};
use crate::types::common::ListParamsReq;

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct CreateInviteRecordReq {
    pub user_id: i32,
    pub inviter_id: i32,
    pub user_info: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Validate)]
pub struct UpdateInviteRecordReq {
    pub user_id: Option<i32>,
    pub inviter_id: Option<i32>,
    pub user_info: Option<serde_json::Value>,
}

#[derive(Deserialize, ToSchema, Debug, Default, IntoParams)]
pub struct SearchInviteRecordsParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(default)]
    pub user_id: Option<i32>,
    #[serde(default)]
    pub inviter_id: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug,FromQueryResult)]
pub struct InviteRecordInfo {
    pub id: i32,
    pub user_id: i32,
    pub inviter_id: i32,
    pub user_info: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub user_username: Option<String>,
    pub inviter_username: Option<String>,
}
