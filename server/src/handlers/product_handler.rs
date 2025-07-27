use crate::types::product_types::*;
use entity::products;

crate::impl_crud_handlers!(
    ProductHandler,
    products::Entity,
    products::ActiveModel,
    products::Model,
    ProductCreatePayload,
    ProductUpdatePayload,
    ListProductsParams,
    products::Model,
    "products",
    true
);

impl CrudOperations for ProductHandler {
    type Entity = products::Entity;
    type CreatePayload = ProductCreatePayload;
    type UpdatePayload = ProductUpdatePayload;
    type SearchPayLoad = ListProductsParams;
    type ActiveModel = products::ActiveModel;
    type Model = products::Model;
    type SearchResult = products::Model;
    type QueryResult = sea_orm::Select<products::Entity>;

    fn table_name() -> &'static str {
        "products"
    }

    fn create_model(payload: Self::CreatePayload) -> Result<Self::ActiveModel, AppError> {
        Ok(products::ActiveModel {
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
        })
    }

    fn update_model(
        payload: Self::UpdatePayload,
        product: products::Model,
    ) -> Result<Self::ActiveModel, AppError> {
        let mut product: products::ActiveModel = product.into_active_model();
        crate::update_field_if_some!(product, name, payload.name);
        crate::update_field_if_some!(product, price, payload.price);
        crate::update_field_if_some!(product, app_id, payload.app_id);
        crate::update_field_if_some!(product, product_id, payload.product_id);
        crate::update_field_if_some!(product, add_valid_days, payload.add_valid_days);
        crate::update_field_if_some!(product, image_url, payload.image_url, option);
        crate::update_field_if_some!(product, tags, payload.tags, option);
        crate::update_field_if_some!(product, remark, payload.remark, option);
        crate::update_field_if_some!(product, status, payload.status);
        Ok(product)
    }

    fn get_list(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let mut query = products::Entity::find()
            .filter(products::Column::DeletedAt.is_null())
            .order_by_desc(products::Column::CreatedAt);

        crate::filter_if_some!(query, products::Column::Id, payload.id, eq);
        crate::filter_if_some!(
            query,
            products::Column::ProductId,
            payload.product_id,
            contains
        );
        crate::filter_if_some!(query, products::Column::Name, payload.name, contains);
        Ok(query)
    }

    fn get_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::get_list(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
}
