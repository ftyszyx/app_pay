use crate::types::user_types::*;
use entity::roles;
use entity::users;
use migration::SubQueryStatement;
use sea_orm::QuerySelect;
use sea_orm::sea_query::{Expr, Query as SeaQuery, SimpleExpr};
use uuid::Uuid;

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
            user_id: Set(Uuid::new_v4().to_string()),
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
        let subquery = SeaQuery::select()
            .expr(Expr::col(entity::invite_records::Column::Id).count())
            .from(entity::invite_records::Entity)
            .and_where(
                Expr::col(entity::invite_records::Column::InviterId)
                    .eq(Expr::col(entity::users::Column::Id)),
            )
            .to_owned();
        let mut query = users::Entity::find()
            .find_also_related(roles::Entity)
            .filter(users::Column::DeletedAt.is_null())
            .column_as(
                SimpleExpr::SubQuery(None, Box::new(SubQueryStatement::SelectStatement(subquery))),
                "invite_count",
            )
            .order_by_asc(users::Column::Id);
        // query.expr_as(subquery, "invite_count");
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
#[allow(dead_code)]
async fn test_find(state: &AppState) -> Result<(), AppError> {
    let query = UserHandler::build_query_by_id(3)?;
    let paginator = query.paginate(&state.db, 10);
    let count = paginator.num_items().await?;
    println!("count: {}", count);
    let user = paginator.fetch_page(1).await?;
    println!("{:?}", user);
    Ok(())
}

#[allow(dead_code)]
async fn test_findone(state: &AppState) -> Result<(), AppError> {
    let query = UserHandler::build_query_by_id(3)?;
    let user = query.one(&state.db).await?;
    println!("{:?}", user);
    Ok(())
}
