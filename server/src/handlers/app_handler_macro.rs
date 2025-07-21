use entity::apps;
use crate::types::app_types::*;

// 使用宏生成 CRUD handlers
crate::impl_crud_handlers!(
    apps,
    apps::Entity,
    apps::Model,
    apps::ActiveModel,
    AddAppReq,
    UpdateAppReq,
    ListAppsParams,
    // 创建逻辑
    {
        apps::ActiveModel {
            name: Set(payload.name),
            app_id: Set(payload.app_id),
            app_vername: Set(payload.app_vername),
            app_vercode: Set(payload.app_vercode),
            app_download_url: Set(payload.app_download_url),
            app_res_url: Set(payload.app_res_url),
            app_update_info: Set(payload.app_update_info),
            sort_order: Set(payload.sort_order),
            status: Set(payload.status),
            ..Default::default()
        }
    },
    // 更新逻辑
    {
        if let Some(name) = req.name {
            active_model.name = Set(name);
        }
        if let Some(app_id) = req.app_id {
            active_model.app_id = Set(app_id);
        }
        if let Some(app_vername) = req.app_vername {
            active_model.app_vername = Set(app_vername);
        }
        if let Some(app_vercode) = req.app_vercode {
            active_model.app_vercode = Set(app_vercode);
        }
        if let Some(app_download_url) = req.app_download_url {
            active_model.app_download_url = Set(app_download_url);
        }
        if let Some(app_res_url) = req.app_res_url {
            active_model.app_res_url = Set(app_res_url);
        }
        if let Some(app_update_info) = req.app_update_info {
            active_model.app_update_info = Set(app_update_info);
        }
        if let Some(sort_order) = req.sort_order {
            active_model.sort_order = Set(sort_order);
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }
    },
    // 列表过滤逻辑
    {
        if let Some(name) = &payload.name {
            if !name.is_empty() {
                query = query.filter(apps::Column::Name.contains(name));
            }
        }
    }
); 