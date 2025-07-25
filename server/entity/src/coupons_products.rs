use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "coupons_products")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub coupon_id: i32,
    pub product_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::coupons::Entity",
        from = "Column::CouponId",
        to = "super::coupons::Column::Id"
    )]
    Coupons,
    #[sea_orm(
        belongs_to = "super::products::Entity",
        from = "Column::ProductId",
        to = "super::products::Column::Id"
    )]
    Products,
}

impl Related<super::coupons::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Coupons.def()
    }
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Products.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
