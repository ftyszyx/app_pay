use crate::{
    database::*,
    my_error::MyError,
    types::{app_types::*, common::*},
};
use axum::{Json, extract::State};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde_json::json;
//test

pub async fn add_app(
    State(state): State<AppState>,
    Json(req): Json<AddAppReq>,
) -> Result<ApiResult<apps::Model>, MyError> {
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
    let app = new_app.insert(&state.db).await?;
    Ok(ApiResult::success(app))
}

pub async fn update_app(
    State(state): State<AppState>,
    Json(req): Json<UpdateAppReq>,
) -> Result<ApiResult<apps::Model>, MyError> {
    let app = apps::Entity::find_by_id(req.id).one(&state.db).await?;
    if let Some(app) = app {
        let mut app = app.into_active_model();
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
            app.app_update_info = Set(app_update_info);
        }
        if let Some(sort_order) = req.sort_order {
            app.sort_order = Set(sort_order);
        }
        if let Some(status) = req.status {
            app.status = Set(status);
        }
        let app = app.update(&state.db).await?;
        Ok(ApiResult::success(app))
    } else {
        Err(MyError::new("app not found"))
    }
}

pub async fn delete_app(
    State(state): State<AppState>,
    Json(req): Json<DelReq>,
) -> Result<ApiResult<serde_json::Value>, MyError> {
    let result = apps::Entity::delete_by_id(req.id).exec(&state.db).await?;
    if result.rows_affected == 1 {
        Ok(ApiResult::success(json!({ "message": "success" })))
    } else {
        Err(MyError::new("app not found or delete failed"))
    }
}

pub async fn get_app_list(
    State(state): State<AppState>,
) -> Result<ApiResult<Vec<apps::Model>>, MyError> {
    let apps = apps::Entity::find().all(&state.db).await?;
    Ok(ApiResult::success(apps))
}

pub async fn get_app_by_id(
    State(state): State<AppState>,
    Json(req): Json<GetReq>,
) -> Result<ApiResult<apps::Model>, MyError> {
    let app = apps::Entity::find_by_id(req.id).one(&state.db).await?;
    if let Some(app) = app {
        Ok(ApiResult::success(app))
    } else {
        Err(MyError::new("app not found"))
    }
}
