use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: DateTime,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub balance: Decimal,
    pub inviter_id: Option<i32>,
    pub invite_count: i32,
    #[sea_orm(column_type = "Decimal(Some((10, 2)))")]
    pub invite_rebate_total: Decimal,
    pub role_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::role::Entity",
        from = "Column::RoleId",
        to = "super::role::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Role,
}

impl Related<super::role::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
