use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::Utc;
use entity::apps;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde_json::json;

use crate::{
    types::response::ApiResponse,
    types::{app_types::*, common::*},
};

#[utoipa::path(
    post,
    path = "/api/admin/apps",
    security( ("api_key" = [])),
    request_body = AddAppReq,
    responses(
        (status = 200, description = "Success", body = apps::Model),
    )
)]
pub async fn add_app(
    State(db): State<sea_orm::DatabaseConnection>,
    Json(req): Json<AddAppReq>,
) -> Result<ApiResponse<apps::Model>, AppError> {
    let new_app = apps::ActiveModel {
        name: Set(req.name),
        app_id: Set(req.app_id),
        app_vername: Set(req.app_vername),
        app_vercode: Set(req.app_vercode),
        app_download_url: Set(req.app_download_url),
        app_res_url: Set(req.app_res_url),
        app_update_info: Set(req.app_update_info),
        sort_order: Set(req.sort_order),
        status: Set(req.status),
        ..Default::default()
    };

    let app = new_app.insert(&db).await?;
    Ok(ApiResponse::success(app))
}

#[utoipa::path(
    put,
    path = "/api/admin/apps/{id}",
    security( ("api_key" = [])),
    request_body = UpdateAppReq,
    responses(
        (status = 200, description = "Success", body = apps::Model),
    )
)]
pub async fn update_app(
    State(db): State<sea_orm::DatabaseConnection>,
    Json(req): Json<UpdateAppReq>,
) -> Result<ApiResponse<apps::Model>, AppError> {
    let app = apps::Entity::find_by_id(req.id).one(&db).await?;
    let app = app.ok_or(AppError::AppNotFound)?;
    let mut app: apps::ActiveModel = app.into_active_model();
    if let Some(name) = req.name {
        app.name = Set(name);
    }
    if let Some(app_id) = req.app_id {
        app.app_id = Set(app_id);
    }
    if let Some(app_vername) = req.app_vername {
        app.app_vername = Set(app_vername);
    }
    if let Some(app_vercode) = req.app_vercode {
        app.app_vercode = Set(app_vercode);
    }
    if let Some(app_download_url) = req.app_download_url {
        app.app_download_url = Set(app_download_url);
    }
    if let Some(app_res_url) = req.app_res_url {
        app.app_res_url = Set(app_res_url);
    }
    if let Some(app_update_info) = req.app_update_info {
        app.app_update_info = Set(Some(app_update_info));
    }
    if let Some(sort_order) = req.sort_order {
        app.sort_order = Set(sort_order);
    }
    if let Some(status) = req.status {
        app.status = Set(status);
    }

    let app = app.update(&db).await?;
    Ok(ApiResponse::success(app))
}

#[utoipa::path(
    delete,
    path = "/api/admin/apps/{id}",
    security( ("api_key" = [])),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
    )
)]
pub async fn delete_app(
    State(db): State<sea_orm::DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    let app = apps::Entity::find_by_id(id).one(&db).await?;
    let app = app.ok_or(AppError::AppNotFound)?;
    let mut app: apps::ActiveModel = app.into_active_model();
    app.deleted_at = Set(Some(Utc::now().naive_utc()));
    app.update(&db).await?;
    Ok(ApiResponse::success(json!({ "message": "success" })))
}

#[utoipa::path(
    get,
    path = "/api/admin/apps",
    security( ("api_key" = [])),
    responses(
        (status = 200, description = "Success", body = PagingResponse<apps::Model>),
    )
)]
pub async fn get_app_list(
    State(db): State<sea_orm::DatabaseConnection>,
    Query(params): Query<ListParamsReq>,
    Json(payload): Json<ListAppsParams>,
) -> Result<ApiResponse<PagingResponse<apps::Model>>, AppError> {
    let page = params.page;
    let page_size = params.page_size;
    let mut query = apps::Entity::find()
        .filter(apps::Column::DeletedAt.is_null())
        .order_by_desc(apps::Column::CreatedAt);
    if let Some(name) = payload.name.filter(|n| !n.is_empty()) {
        query = query.filter(apps::Column::Name.contains(&name));
    }
    let paginator = query.paginate(&db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let apps = paginator.fetch_page(page - 1).await?;
    Ok(ApiResponse::success(PagingResponse {
        list: apps,
        total,
        page,
    }))
}

#[utoipa::path(
    get,
    path = "/api/admin/apps/{id}",
    security( ("api_key" = [])),
    responses(
        (status = 200, description = "Success", body = apps::Model),
    )
)]
pub async fn get_app_by_id(
    State(db): State<sea_orm::DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<apps::Model>, AppError> {
    let app = apps::Entity::find_by_id(id)
        .filter(apps::Column::DeletedAt.is_null())
        .one(&db)
        .await?;
    let app = app.ok_or(AppError::AppNotFound)?;
    Ok(ApiResponse::success(app))
}
