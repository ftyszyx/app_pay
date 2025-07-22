use crate::types::product_types::{ListProductsParams, ProductCreatePayload, ProductUpdatePayload};
use entity::products;
crate::import_crud_macro!();

pub fn create_product_model(payload: ProductCreatePayload) -> products::ActiveModel {
    products::ActiveModel {
        name: Set(payload.name),
        price: Set(payload.price),
        app_id: Set(payload.app_id),
        product_id: Set(payload.product_id),
        add_valid_days: Set(payload.add_valid_days),
        image_url: Set(payload.image_url),
        tags: Set(payload.tags),
        status: Set(payload.status),
        remark: Set(payload.remark),
        ..Default::default()
    }
}

pub fn update_product_model(
    payload: ProductUpdatePayload,
    product: products::Model,
) -> products::ActiveModel {
    let mut product: products::ActiveModel = product.into_active_model();
    if let Some(name) = payload.name {
        product.name = Set(name);
    }
    if let Some(price) = payload.price {
        product.price = Set(price);
    }
    if let Some(app_id) = payload.app_id {
        product.app_id = Set(app_id);
    }
    if let Some(product_id) = payload.product_id {
        product.product_id = Set(product_id);
    }
    if let Some(add_valid_days) = payload.add_valid_days {
        product.add_valid_days = Set(add_valid_days);
    }
    if let Some(image_url) = payload.image_url {
        product.image_url = Set(Some(image_url));
    }
    if let Some(tags) = payload.tags {
        product.tags = Set(Some(tags));
    }
    if let Some(status) = payload.status {
        product.status = Set(status);
    }
    if let Some(remark) = payload.remark {
        product.remark = Set(Some(remark));
    }
    product
}

pub fn get_product_list_query(payload: ListProductsParams) -> sea_orm::Select<products::Entity> {
    let mut query = products::Entity::find()
        .filter(products::Column::DeletedAt.is_null())
        .order_by_asc(products::Column::Id);
    if let Some(name) = payload.name {
        if !name.is_empty() {
            query = query.filter(products::Column::Name.contains(&name));
        }
    }
    query
}

crate::impl_add_handler!(
    product,
    products::Entity,
    ProductCreatePayload,
    products::ActiveModel,
    products::Model,
    create_product_model
);
crate::impl_update_handler!(
    product,
    products::Entity,
    ProductUpdatePayload,
    products::ActiveModel,
    products::Model,
    update_product_model
);
crate::impl_fake_delete_handler!(
    product,
    products::Entity,
    products::ActiveModel,
    products::Model
);
crate::impl_get_handler!(
    product,
    products::Entity,
    ListProductsParams,
    products::Model,
    get_product_list_query
);
crate::impl_get_by_id_handler!(product, products::Entity, products::Model, true);
