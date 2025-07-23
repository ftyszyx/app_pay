use crate::types::pay_method_types::*;
use entity::pay_methods;
crate::impl_crud_handlers!(
    PayMethodHandler,
    pay_methods::Entity,
    pay_methods::ActiveModel,
    pay_methods::Model,
    PayMethodCreatePayload,
    PayMethodUpdatePayload,
    ListPayMethodsParams,
    pay_methods::Model,
    "pay_methods",
    true
);

impl CrudOperations for PayMethodHandler {
    type Entity = pay_methods::Entity;
    type ActiveModel = pay_methods::ActiveModel;
    type Model = pay_methods::Model;
    type CreatePayload = PayMethodCreatePayload;
    type UpdatePayload = PayMethodUpdatePayload;
    type SearchPayLoad = ListPayMethodsParams;
    type SearchResult = pay_methods::Model;
    type QueryResult = sea_orm::Select<pay_methods::Entity>;

    fn table_name() -> &'static str {
        "pay_methods"
    }

    fn create_model(payload: Self::CreatePayload) -> Self::ActiveModel {
        pay_methods::ActiveModel {
            name: Set(payload.name),
            status: Set(payload.status),
            remark: Set(payload.remark),
            config: Set(payload.config),
            ..Default::default()
        }
    }

    fn update_model(payload: Self::UpdatePayload, model: Self::Model) -> Self::ActiveModel {
        let mut model: Self::ActiveModel = model.into_active_model();
        if let Some(name) = payload.name {
            model.name = Set(name);
        }
        model
    }

    fn build_query(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let mut query = pay_methods::Entity::find()
            .filter(pay_methods::Column::DeletedAt.is_null())
            .order_by_asc(pay_methods::Column::Id);
        crate::filter_if_some!(query, pay_methods::Column::Name, payload.name, contains);
        crate::filter_if_some!(query, pay_methods::Column::Id, payload.id, eq);
        Ok(query)
    }

    fn build_query_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::build_query(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
}
