use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use uuid::Uuid;
use crate::{constants, handlers::response::ApiResponse, my_error::ErrorCode};
use entity::{invite_records, roles, users};
use futures::future::join_all;
use crate::types::user_types::{ListUsersParams, UserCreatePayload, UserInfo, UserListResponse, UserUpdatePayload};
use crate::types::common::AppError;

pub async fn model_to_info(u: users::Model, db: &DatabaseConnection) -> Result<UserInfo, AppError> {
    let (role_id, role_name) = {
       match roles::Entity::find_by_id(u.role_id).one(db).await {
            Ok(Some(role)) => (role.id, role.name),
            Ok(None) => return Err(AppError::UserNotFound),
            Err(e) => return Err(e.into()),
        }
    };

    let invite_count = invite_records::Entity::find()
        .filter(invite_records::Column::InviterId.eq(u.id))
        .count(db)
        .await? as i32;
    let balance = u.balance.to_string();
    let invite_rebate_total = u.invite_rebate_total;
    let created_at = u.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
    Ok(UserInfo {
        id: u.id,
        username: u.username,
        balance,
        inviter_id: u.inviter_id,
        invite_count,
        invite_rebate_total,
        role_id,
        role_name,
        created_at,
    })
}

#[utoipa::path(
    post,
    path = "/api/admin/users",
    request_body = UserCreatePayload,
    responses(
        (status = 200, description = "User created successfully", body = ApiResponse<UserInfo>),
        (status = 409, description = "User already exists"),
        (status = 500, description = "Internal server error")
    ),
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
        Ok(user) => match model_to_info(user, &db).await {
            Ok(user_info) => ApiResponse::success(user_info),
            Err(app_err) => ApiResponse::<UserInfo>::error_with_message(app_err.to_string()),
        },
        Err(_) => ApiResponse::<UserInfo>::error_with_code(ErrorCode::UserAlreadyExists),
    }
}

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = ApiResponse<UserListResponse>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_users_list(
    State(db): State<DatabaseConnection>,
    Query(params): Query<ListUsersParams>,
) -> impl IntoResponse {
    // Implementation for getting a paginated and filtered list of users
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let mut query = users::Entity::find().order_by_desc(users::Column::Id);

    if let Some(username) = params.username {
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

    let mut list = Vec::with_capacity(results.len());
    for result in results {
        match result {
            Ok(user_info) => list.push(user_info),
            Err(app_err) => {
                return ApiResponse::<UserListResponse>::error_with_message(app_err.to_string())
            }
        }
    }
    ApiResponse::success(UserListResponse { list, total })
}

#[utoipa::path(
    get,
    path = "/api/admin/users/{id}",
    responses(
        (status = 200, description = "User found", body = ApiResponse<UserInfo>),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_user_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match users::Entity::find_by_id(id).one(&db).await {
        Ok(Some(user)) => match model_to_info(user, &db).await {
            Ok(user_info) => ApiResponse::success(user_info),
            Err(app_err) => ApiResponse::<UserInfo>::error_with_message(app_err.to_string()),
        },
        Ok(None) => ApiResponse::<UserInfo>::error_with_code(ErrorCode::UserNotFound),
        Err(db_err) => ApiResponse::<UserInfo>::error_with_message(db_err.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/api/admin/users/{id}",
    request_body = UserUpdatePayload,
    responses(
        (status = 200, description = "User updated successfully", body = ApiResponse<UserInfo>),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn update_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdatePayload>,
) -> impl IntoResponse {
    let mut user: users::ActiveModel = match users::Entity::find_by_id(id).one(&db).await {
        Ok(Some(user_model)) => user_model.into(),
        _ => return ApiResponse::<UserInfo>::error_with_code(ErrorCode::UserNotFound),
    };

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

    match user.update(&db).await {
        Ok(user) => match model_to_info(user, &db).await {
            Ok(user_info) => ApiResponse::success(user_info),
            Err(app_err) => ApiResponse::<UserInfo>::error_with_message(app_err.to_string()),
        },
        Err(db_err) => ApiResponse::<UserInfo>::error_with_message(db_err.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/api/admin/users/{id}",
    responses(
        (status = 200, description = "User deleted successfully"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut user: users::ActiveModel = match users::Entity::find_by_id(id).one(&db).await {
        Ok(Some(user)) => user.into(),
        Ok(None) => return ApiResponse::<()>::error_with_code(ErrorCode::UserNotFound),
        Err(db_err) => return ApiResponse::<()>::error_with_message(db_err.to_string()),
    };
    user.deleted_at = Set(Some(Utc::now().naive_utc()));
    match user.update(&db).await {
        Ok(_) => ApiResponse::success(()),
        Err(db_err) => ApiResponse::<()>::error_with_message(db_err.to_string()),
    }
}
