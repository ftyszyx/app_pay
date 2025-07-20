use crate::{handlers::response::ApiResponse, my_error::ErrorCode};
use crate::types::role_types::{RoleCreatePayload, RoleListResponse, RoleUpdatePayload};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use entity::roles;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListRolesParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[utoipa::path(
    post,
    path = "/api/admin/roles",
    request_body = RoleCreatePayload,
    responses(
        (status = 200, description = "Role created successfully", body = ApiResponse<roles::Model>),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn create_role(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RoleCreatePayload>,
) -> impl IntoResponse {
    let new_role = roles::ActiveModel {
        name: Set(payload.name),
        remark: Set(payload.remark),
        ..Default::default()
    };
    match new_role.insert(&db).await {
        Ok(role) => ApiResponse::success(role),
        Err(err) => ApiResponse::<roles::Model>::error_with_message(err.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/roles",
    responses( (status = 200, description = "List of roles", body = ApiResponse<RoleListResponse>),),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_roles_list(
    State(db): State<DatabaseConnection>,
    Query(params): Query<ListRolesParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);
    let paginator = roles::Entity::find()
        .order_by_asc(roles::Column::Id)
        .paginate(&db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    match paginator.fetch_page(page - 1).await {
        Ok(list) => ApiResponse::success(RoleListResponse { list, total }),
        Err(err) => ApiResponse::<RoleListResponse>::error_with_message(err.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/api/admin/roles/{id}",
    responses(
        (status = 200, description = "Role found", body = ApiResponse<roles::Model>),
        (status = 404, description = "Role not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_role_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match roles::Entity::find_by_id(id).one(&db).await {
        Ok(Some(role)) => ApiResponse::success(role),
        Ok(None) => ApiResponse::<roles::Model>::error_with_code(ErrorCode::UserNotFound),
        Err(err) => ApiResponse::<roles::Model>::error_with_message(err.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/api/admin/roles/{id}",
    request_body = RoleUpdatePayload,
    responses(
        (status = 200, description = "Role updated successfully", body = ApiResponse<roles::Model>),
        (status = 404, description = "Role not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn update_role(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<RoleUpdatePayload>,
) -> impl IntoResponse {
    let mut role: roles::ActiveModel = match roles::Entity::find_by_id(id).one(&db).await {
        Ok(Some(role)) => role.into(),
        Ok(None) => return ApiResponse::<roles::Model>::error_with_code(ErrorCode::UserNotFound),
        Err(err) => return ApiResponse::<roles::Model>::error_with_message(err.to_string()),
    };
    if let Some(name) = payload.name {
        role.name = Set(name);
    }
    if let Some(remark) = payload.remark {
        role.remark = Set(Some(remark));
    }
    match role.update(&db).await {
        Ok(role) => ApiResponse::success(role),
        Err(err) => ApiResponse::<roles::Model>::error_with_message(err.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/api/admin/roles/{id}",
    responses(
        (status = 200, description = "Role deleted successfully"),
        (status = 404, description = "Role not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn delete_role(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match roles::Entity::delete_by_id(id).exec(&db).await {
        Ok(res) => {
            if res.rows_affected == 1 {
                ApiResponse::success(())
            } else {
                ApiResponse::<()>::error_with_code(ErrorCode::UserNotFound)
            }
        }
        Err(err) => ApiResponse::<()>::error_with_message(err.to_string()),
    }
} 