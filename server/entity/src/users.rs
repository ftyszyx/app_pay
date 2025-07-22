use sea_orm::entity::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, ToSchema)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub created_at: DateTime,
    pub deleted_at: Option<DateTime>,
    pub balance: i64,
    pub inviter_id: Option<i32>,
    pub invite_rebate_total: i64,
    pub role_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::roles::Entity",
        from = "Column::RoleId",
        to = "super::roles::Column::Id"
    )]
    Roles,
    #[sea_orm(belongs_to = "Entity", from = "Column::InviterId", to = "Column::Id")]
    SelfRef,
    #[sea_orm(has_many = "Entity")]
    InvitedUsers,
    #[sea_orm(has_many = "super::orders::Entity")]
    Orders,
    #[sea_orm(has_many = "super::invite_records::Entity")]
    InviteRecords,
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Roles.def()
    }
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::invite_records::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::InviteRecords.def()
    }
}

// Implement self-relation for users::Entity
impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SelfRef.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
