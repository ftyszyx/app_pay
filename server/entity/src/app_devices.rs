//! `SeaORM` Entity, handwritten for app_devices table

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "app_devices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub app_id: i32,
    pub device_id: String,
    pub device_info: Option<Json>,
    pub bind_time: Option<DateTime<Utc>>,    
    pub expire_time: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::apps::Entity",
        from = "Column::AppId",
        to = "super::apps::Column::Id"
    )]
    Apps,
}

impl Related<super::apps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Apps.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


