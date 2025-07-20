use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "order_coupons")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub order_id: i32,
    pub coupon_id: i32,
    pub num: i32,
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
        belongs_to = "super::coupons::Entity",
        from = "Column::CouponId",
        to = "super::coupons::Column::Id"
    )]
    Coupons,
}

impl Related<super::orders::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Orders.def()
    }
}

impl Related<super::coupons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Coupons.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 