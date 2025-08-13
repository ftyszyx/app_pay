//! `SeaORM` Entity for resources

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(
    Deserialize, Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize, utoipa::ToSchema,
)]
#[sea_orm(table_name = "resources")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub object_key: String,
    pub url: String,
    pub res_type: i16,
    pub path: String,
    pub tags: Option<Vec<String>>,
    pub status: i16,
    pub remark: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

