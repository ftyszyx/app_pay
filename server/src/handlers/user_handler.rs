use crate::types::user_types::{UserCreatePayload, UserInfo, UserUpdatePayload};
use crate::{constants, types::user_types::ListUsersParams};
use entity::users;
crate::import_crud_macro!();

fn create_user_model(payload: UserCreatePayload) -> users::ActiveModel {
    users::ActiveModel {
        username: Set(payload.username),
        password: Set(bcrypt::hash(payload.password, 10).unwrap()),
        role_id: Set(payload.role_id.unwrap_or(constants::DEFAULT_ROLE_ID)),
        ..Default::default()
    }
}

fn update_user_model(payload: UserUpdatePayload, user: users::Model) -> users::ActiveModel {
    let mut user: users::ActiveModel = user.into_active_model();
    if let Some(username) = payload.username {
        user.username = Set(username);
    }
    if let Some(password) = payload.password {
        user.password = Set(bcrypt::hash(password, 10).unwrap());
    }
    if let Some(role_id) = payload.role_id {
        user.role_id = Set(role_id);
    }
    if let Some(balance) = payload.balance {
        user.balance = Set(balance);
    }
    user
}

fn get_user_list_query(payload: ListUsersParams) -> sea_orm::Select<users::Entity> {
    let mut query = users::Entity::find()
        .filter(users::Column::DeletedAt.is_null())
        .order_by_asc(users::Column::Id);
    if let Some(username) = payload.username {
        query = query.filter(users::Column::Username.eq(username));
    }

    query
}

crate::impl_add_handler!(
    user,
    users::Entity,
    UserCreatePayload,
    users::ActiveModel,
    users::Model,
    create_user_model
);
crate::impl_update_handler!(
    user,
    users::Entity,
    UserUpdatePayload,
    users::ActiveModel,
    users::Model,
    update_user_model
);
crate::impl_fake_delete_handler!(user, users::Entity, users::ActiveModel, users::Model);
crate::impl_get_handler!(
    user,
    users::Entity,
    ListUsersParams,
    users::Model,
    get_user_list_query
);
crate::impl_get_by_id_handler!(user, users::Entity, users::Model, true);
