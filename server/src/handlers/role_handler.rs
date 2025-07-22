use crate::types::role_types::*;
use entity::roles;

crate::impl_crud_handlers!(
    RoleHandler,
    roles::Entity,
    roles::ActiveModel,
    roles::Model,
    RoleCreatePayload,
    RoleUpdatePayload,
    ListRolesParams,
    "roles",
    true
);

impl CrudOperations for RoleHandler {
    type Entity = roles::Entity;
    type ActiveModel = roles::ActiveModel;
    type Model = roles::Model;
    type CreatePayload = RoleCreatePayload;
    type UpdatePayload = RoleUpdatePayload;
    type SearchPayLoad = ListRolesParams;
    fn table_name() -> &'static str {
        "roles"
    }

    fn create_model(payload: Self::CreatePayload) -> Self::ActiveModel {
        roles::ActiveModel {
            name: Set(payload.name),
            remark: Set(payload.remark),
            ..Default::default()
        }
    }

    fn update_model(payload: Self::UpdatePayload, role: Self::Model) -> Self::ActiveModel {
        let mut role: Self::ActiveModel = role.into_active_model();
        if let Some(name) = payload.name {
            role.name = Set(name);
        }
        role
    }

    fn build_query(payload: Self::SearchPayLoad) -> sea_orm::Select<Self::Entity> {
        let mut query = roles::Entity::find()
            .filter(roles::Column::DeletedAt.is_null())
            .order_by_asc(roles::Column::Id);
        if let Some(name) = payload.name {
            query = query.filter(roles::Column::Name.eq(name));
        }
        query
    }
}
