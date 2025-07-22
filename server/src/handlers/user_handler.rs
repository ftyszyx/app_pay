use crate::types::user_types::*;
use entity::users;

// User Handler - 使用新的统一CRUD架构
crate::impl_crud_handlers!(
    UserHandler,
    users::Entity,
    users::ActiveModel,
    users::Model,
    UserCreatePayload,
    UserUpdatePayload,
    ListUsersParams,
    "users",
    true
);

impl CrudOperations for UserHandler {
    type Entity = users::Entity;
    type CreatePayload = UserCreatePayload;
    type UpdatePayload = UserUpdatePayload;
    type ListPayload = ListUsersParams;
    type ActiveModel = users::ActiveModel;
    type Model = users::Model;
    fn table_name() -> &'static str {
        "users"
    }

    fn create_model(payload: Self::CreatePayload) -> users::ActiveModel {
        users::ActiveModel {
            username: Set(payload.username),
            password: Set(bcrypt::hash(payload.password, 10).unwrap()),
            role_id: Set(payload.role_id.unwrap_or(crate::constants::DEFAULT_ROLE_ID)),
            ..Default::default()
        }
    }

    fn update_model(payload: Self::UpdatePayload, user: users::Model) -> users::ActiveModel {
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

    fn build_query(payload: Self::ListPayload) -> sea_orm::Select<users::Entity> {
        let mut query = users::Entity::find()
            .filter(users::Column::DeletedAt.is_null())
            .order_by_asc(users::Column::Id);

        if let Some(username) = payload.username {
            if !username.is_empty() {
                query = query.filter(users::Column::Username.contains(&username));
            }
        }
        query
    }
}
