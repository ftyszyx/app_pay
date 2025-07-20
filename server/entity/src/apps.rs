use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "apps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub app_id: String,
    pub app_vername: String,
    pub app_vercode: i32,
    pub app_download_url: String,
    pub app_res_url: String,
    pub app_update_info: Option<String>,
    pub sort_order: i32,
    pub status: i16,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::products::Entity")]
    Products,
    #[sea_orm(has_many = "super::reg_codes::Entity")]
    RegCodes,
    #[sea_orm(has_many = "super::coupons_apps::Entity")]
    CouponsApps,
}

impl Related<super::products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Products.def()
    }
}

impl Related<super::reg_codes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RegCodes.def()
    }
}

impl Related<super::coupons_apps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CouponsApps.def()
    }
}

impl ActiveModelBehavior for ActiveModel {} 