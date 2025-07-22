use crate::types::pay_method_types::*;
use entity::pay_methods;
crate::import_crud_macro!();

fn create_pay_method_model(payload: PayMethodCreatePayload) -> pay_methods::ActiveModel {
    pay_methods::ActiveModel {
        name: Set(payload.name),
        status: Set(payload.status),
        remark: Set(payload.remark),
        config: Set(payload.config),
        ..Default::default()
    }
}

fn update_pay_method_model(
    payload: PayMethodUpdatePayload,
    pay_method: pay_methods::Model,
) -> pay_methods::ActiveModel {
    let mut pay_method: pay_methods::ActiveModel = pay_method.into_active_model();
    if let Some(name) = payload.name {
        pay_method.name = Set(name);
    }
    pay_method
}

fn get_pay_method_list_query(
    payload: ListPayMethodsParams,
) -> sea_orm::Select<pay_methods::Entity> {
    let mut query = pay_methods::Entity::find()
        .filter(pay_methods::Column::DeletedAt.is_null())
        .order_by_asc(pay_methods::Column::Id);
    if let Some(name) = payload.name {
        query = query.filter(pay_methods::Column::Name.eq(name));
    }
    query
}

crate::impl_add_handler!(
    pay_method,
    pay_methods::Entity,
    PayMethodCreatePayload,
    pay_methods::ActiveModel,
    pay_methods::Model,
    create_pay_method_model
);

crate::impl_update_handler!(
    pay_method,
    pay_methods::Entity,
    PayMethodUpdatePayload,
    pay_methods::ActiveModel,
    pay_methods::Model,
    update_pay_method_model
);

crate::impl_get_handler!(
    pay_method,
    pay_methods::Entity,
    ListPayMethodsParams,
    pay_methods::Model,
    get_pay_method_list_query
);

crate::impl_get_by_id_handler!(pay_method, pay_methods::Entity, pay_methods::Model, true);

crate::impl_fake_delete_handler!(
    pay_method,
    pay_methods::Entity,
    pay_methods::ActiveModel,
    pay_methods::Model
);
