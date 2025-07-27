use crate::types::orders_types::{CreateOrderReq, UpdateOrderReq, SearchOrdersParams, OrderInfo};
use entity::orders;
use entity::pay_methods;
use entity::users;
use sea_orm::QuerySelect;

// Order Handler - 使用新的统一CRUD架构
crate::impl_crud_handlers!(
    OrderHandler,
    orders::Entity,
    orders::ActiveModel,
    orders::Model,
    CreateOrderReq,
    UpdateOrderReq,
    SearchOrdersParams,
    OrderInfo,
    "orders",
    false
);

impl CrudOperations for OrderHandler {
    type Entity = orders::Entity;
    type CreatePayload = CreateOrderReq;
    type UpdatePayload = UpdateOrderReq;
    type SearchPayLoad = SearchOrdersParams;
    type SearchResult = OrderInfo;
    type ActiveModel = orders::ActiveModel;
    type Model = orders::Model;
    type QueryResult = sea_orm::Select<orders::Entity>;

    fn table_name() -> &'static str {
        "orders"
    }

    fn create_model(payload: Self::CreatePayload) -> Result<Self::ActiveModel, AppError> {
        Ok(orders::ActiveModel {
            order_id: Set(payload.order_id),
            user_info: Set(payload.user_info),
            status: Set(payload.status),
            pay_method_id: Set(payload.pay_method_id),
            original_price: Set(payload.original_price),
            final_price: Set(payload.final_price),
            remark: Set(payload.remark),
            created_by: Set(payload.created_by),
            updated_by: Set(payload.updated_by),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
    }

    fn update_model(
        payload: Self::UpdatePayload,
        order: orders::Model,
    ) -> Result<Self::ActiveModel, AppError> {
        let mut order: orders::ActiveModel = order.into_active_model();
        crate::update_field_if_some!(order, order_id, payload.order_id);
        crate::update_field_if_some!(order, user_info, payload.user_info, option);
        crate::update_field_if_some!(order, status, payload.status);
        crate::update_field_if_some!(order, pay_method_id, payload.pay_method_id);
        crate::update_field_if_some!(order, original_price, payload.original_price);
        crate::update_field_if_some!(order, final_price, payload.final_price);
        crate::update_field_if_some!(order, remark, payload.remark, option);
        crate::update_field_if_some!(order, updated_by, payload.updated_by);
        order.updated_at = Set(Utc::now());
        Ok(order)
    }

    fn get_list(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let mut query = orders::Entity::find()
            .order_by_desc(orders::Column::CreatedAt);

        crate::filter_if_some!(query, orders::Column::Id, payload.id, eq);
        crate::filter_if_some!(query, orders::Column::OrderId, payload.order_id, contains);
        crate::filter_if_some!(query, orders::Column::Status, payload.status, eq);
        crate::filter_if_some!(query, orders::Column::PayMethodId, payload.pay_method_id, eq);
        crate::filter_if_some!(query, orders::Column::CreatedBy, payload.created_by, eq);
        
        Ok(query)
    }

    fn get_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::get_list(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
} 