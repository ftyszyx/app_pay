use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order_reg_codes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub reg_code_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::orders::Entity",
        from = "Column::OrderId",
        to = "super::orders::Column::Id"
    )]
    Orders,
    #[sea_orm(
        belongs_to = "super::reg_codes::Entity",
        from = "Column::RegCodeId",
        to = "super::reg_codes::Column::Id"
    )]
    RegCodes,
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::reg_codes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RegCodes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 