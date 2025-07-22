use crate::types::role_types::{ListRolesParams, RoleCreatePayload, RoleUpdatePayload};
use entity::roles;
crate::import_crud_macro!();
fn create_role_model(payload: RoleCreatePayload) -> roles::ActiveModel {
    roles::ActiveModel {
        name: Set(payload.name),
        remark: Set(payload.remark),
        ..Default::default()
    }
}

fn update_role_model(payload: RoleUpdatePayload, role: roles::Model) -> roles::ActiveModel {
    let mut role: roles::ActiveModel = role.into_active_model();
    if let Some(name) = payload.name {
        role.name = Set(name);
    }
    role
}

fn get_role_list_query(payload: ListRolesParams) -> sea_orm::Select<roles::Entity> {
    let mut query = roles::Entity::find()
        .filter(roles::Column::DeletedAt.is_null())
        .order_by_asc(roles::Column::Id);
    if let Some(name) = payload.name {
        if !name.is_empty() {
            query = query.filter(roles::Column::Name.contains(&name));
        }
    }
    query
}

crate::impl_add_handler!(
    role,
    roles::Entity,
    RoleCreatePayload,
    roles::ActiveModel,
    roles::Model,
    create_role_model
);
crate::impl_update_handler!(
    role,
    roles::Entity,
    RoleUpdatePayload,
    roles::ActiveModel,
    roles::Model,
    update_role_model
);
crate::impl_fake_delete_handler!(role, roles::Entity, roles::ActiveModel, roles::Model);
crate::impl_get_handler!(
    role,
    roles::Entity,
    ListRolesParams,
    roles::Model,
    get_role_list_query
);
crate::impl_get_by_id_handler!(role, roles::Entity, roles::Model, true);
