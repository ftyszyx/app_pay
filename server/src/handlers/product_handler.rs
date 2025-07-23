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

    fn create_model(payload: Self::CreatePayload) -> products::ActiveModel {
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

    fn update_model(
        payload: Self::UpdatePayload,
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

    fn build_query(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
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

    fn build_query_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::build_query(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
}
