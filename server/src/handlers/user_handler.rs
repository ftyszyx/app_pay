use crate::types::user_types::*;
use entity::roles;
use entity::users;

// User Handler - 使用新的统一CRUD架构
crate::impl_crud_handlers!(
    UserHandler,
    users::Entity,
    users::ActiveModel,
    users::Model,
    UserCreatePayload,
    UserUpdatePayload,
    SearchUsersParams,
    UserInfo,
    "users",
    true
);

impl CrudOperations for UserHandler {
    type Entity = users::Entity;
    type CreatePayload = UserCreatePayload;
    type UpdatePayload = UserUpdatePayload;
    type SearchPayLoad = SearchUsersParams;
    type SearchResult = UserInfo;
    type ActiveModel = users::ActiveModel;
    type Model = users::Model;
    type QueryResult = sea_orm::SelectTwo<users::Entity, roles::Entity>;
    fn table_name() -> &'static str {
        "users"
    }

    fn create_model(payload: Self::CreatePayload) -> Result<Self::ActiveModel, AppError> {
        let password = bcrypt::hash(payload.password, 10)?;
        Ok(users::ActiveModel {
            username: Set(payload.username),
            password: Set(password),
            role_id: Set(payload.role_id.unwrap_or(crate::constants::DEFAULT_ROLE_ID)),
            ..Default::default()
        })
    }

    fn update_model(
        payload: Self::UpdatePayload,
        user: users::Model,
    ) -> Result<Self::ActiveModel, AppError> {
        let mut user: users::ActiveModel = user.into_active_model();
        crate::update_field_if_some!(user, username, payload.username);
        crate::update_field_if_some!(
            user,
            password,
            payload.password,
            with | p | bcrypt::hash(p, 10).unwrap()
        );
        crate::update_field_if_some!(user, role_id, payload.role_id);
        crate::update_field_if_some!(user, balance, payload.balance);
        Ok(user)
    }

    fn build_query(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let mut query = users::Entity::find()
            .find_also_related(roles::Entity)
            .filter(users::Column::DeletedAt.is_null())
            .order_by_asc(users::Column::Id);
        crate::filter_if_some!(query, users::Column::Id, payload.id, eq);
        crate::filter_if_some!(query, users::Column::Username, payload.username, contains);
        crate::filter_if_some!(query, users::Column::UserId, payload.user_id, eq);
        Ok(query)
    }

    fn build_query_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::build_query(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
}
