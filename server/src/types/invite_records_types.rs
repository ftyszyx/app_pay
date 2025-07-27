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

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct InviteRecordInfo {
    pub id: i32,
    pub user_id: i32,
    pub inviter_id: i32,
    pub user_info: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub user_username: Option<String>,
    pub inviter_username: Option<String>,
}

impl TryFrom<(entity::invite_records::Model, Option<entity::users::Model>)> for InviteRecordInfo {
    type Error = crate::types::error::AppError;

    fn try_from(
        value: (entity::invite_records::Model, Option<entity::users::Model>)
    ) -> Result<Self, Self::Error> {
        let (record, user) = value;
        Ok(Self {
            id: record.id,
            user_id: record.user_id,
            inviter_id: record.inviter_id,
            user_info: record.user_info,
            created_at: record.created_at,
            user_username: user.map(|u| u.username),
            inviter_username: inviter.map(|i| i.username),
        })
    }
}

impl TryFrom<entity::invite_records::Model> for InviteRecordInfo {
    type Error = crate::types::error::AppError;

    fn try_from(record: entity::invite_records::Model) -> Result<Self, Self::Error> {
        Ok(Self {
            id: record.id,
            user_id: record.user_id,
            inviter_id: record.inviter_id,
            user_info: record.user_info,
            created_at: record.created_at,
            user_username: None,
            inviter_username: None,
        })
    }
} 