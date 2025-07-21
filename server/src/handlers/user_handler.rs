use crate::types::common::{AppError, ListParamsReq, PagingResponse};
use crate::types::user_types::{UserCreatePayload, UserInfo, UserUpdatePayload, model_to_info};
use crate::{constants, types::response::ApiResponse, types::user_types::ListUsersParams};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use chrono::Utc;
use entity::users;
use futures::future::join_all;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/api/admin/users",
    request_body = UserCreatePayload,
    responses( (status = 200, description = "User created successfully", body = ApiResponse<UserInfo>),),
    security(
        ("api_key" = [])
    )
)]
pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<UserCreatePayload>,
) -> impl IntoResponse {
    let user_id = Uuid::new_v4().to_string();
    let new_user = users::ActiveModel {
        user_id: Set(user_id),
        username: Set(payload.username),
        // Securely hash the password before saving
        password: Set(bcrypt::hash(payload.password, 10).unwrap()),
        role_id: Set(payload.role_id.unwrap_or(constants::DEFAULT_ROLE_ID)),
        ..Default::default()
    };
    match new_user.insert(&db).await {
        Ok(user) => ApiResponse::success(model_to_info(user, &db).await?),
        Err(err) => ApiResponse::<users::Model>::error_with_message(err.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses( (status = 200, description = "List of users", body = ApiResponse<PagingResponse<UserInfo>>),)
)]
pub async fn get_users_list(
    State(db): State<DatabaseConnection>,
    Query(params): Query<ListParamsReq>,
    Json(payload): Json<ListUsersParams>,
) -> Result<ApiResponse<PagingResponse<UserInfo>>, AppError> {
    let page = params.page;
    let page_size = params.page_size;
    let mut query = users::Entity::find().order_by_desc(users::Column::Id);

    if let Some(username) = payload.username {
        if !username.is_empty() {
            query = query.filter(users::Column::Username.contains(&username));
        }
    }
    let paginator = query.paginate(&db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let models = paginator.fetch_page(page - 1).await.unwrap_or_default();
    let futures: Vec<_> = models
        .into_iter()
        .map(|model| model_to_info(model, &db))
        .collect();
    let results = join_all(futures).await;
    let list: Vec<UserInfo> = results
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();
    Ok(ApiResponse::success(PagingResponse { list, total, page }))
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{id}",
    responses( (status = 200, description = "User found", body = ApiResponse<UserInfo>),),
)]
pub async fn get_user_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<UserInfo>, AppError> {
    let user = users::Entity::find_by_id(id).one(&db).await?;
    let user = user.ok_or(AppError::UserNotFound)?;
    let user_info = model_to_info(user, &db).await?;
    Ok(ApiResponse::success(user_info))
}

#[utoipa::path(
    put,
    path = "/api/admin/users/{id}",
    request_body = UserUpdatePayload,
    responses(
        (status = 200, description = "User updated successfully", body = ApiResponse<UserInfo>),
    ),
)]
pub async fn update_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdatePayload>,
) -> Result<ApiResponse<UserInfo>, AppError> {
    let user = users::Entity::find_by_id(id).one(&db).await?;
    let user = user.ok_or(AppError::UserNotFound)?;
    let mut user: users::ActiveModel = user.into();
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
    let user = user.update(&db).await?;
    let user_info = model_to_info(user, &db).await?;
    Ok(ApiResponse::success(user_info))
}

#[utoipa::path(
    delete,
    path = "/api/admin/users/{id}",
    responses( (status = 200, description = "User deleted successfully"),),
    security( ("api_key" = []))
)]
pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<()>, AppError> {
    let user = users::Entity::find_by_id(id).one(&db).await?;
    let user = user.ok_or(AppError::UserNotFound)?;
    let mut user: users::ActiveModel = user.into();
    user.deleted_at = Set(Some(Utc::now().naive_utc()));
    user.update(&db).await?;
    Ok(ApiResponse::success(()))
}
