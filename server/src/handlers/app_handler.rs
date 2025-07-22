use crate::types::app_types::*;
use entity::apps;

// App Handler - 使用新的统一CRUD架构
crate::impl_crud_handlers!(
    AppHandler,
    apps::Entity,
    apps::ActiveModel,
    apps::Model,
    AddAppReq,
    UpdateAppReq,
    ListAppsParams,
    "apps",
    true
);

impl CrudOperations for AppHandler {
    type Entity = apps::Entity;
    type CreatePayload = AddAppReq;
    type UpdatePayload = UpdateAppReq;
    type SearchPayLoad = ListAppsParams;
    type ActiveModel = apps::ActiveModel;
    type Model = apps::Model;
    fn table_name() -> &'static str {
        "apps"
    }

    fn create_model(req: Self::CreatePayload) -> apps::ActiveModel {
        apps::ActiveModel {
            name: Set(req.name),
            app_id: Set(req.app_id),
            app_vername: Set(req.app_vername),
            app_vercode: Set(req.app_vercode),
            app_download_url: Set(req.app_download_url),
            app_res_url: Set(req.app_res_url),
            app_update_info: Set(req.app_update_info),
            sort_order: Set(req.sort_order),
            created_at: Set(Utc::now().naive_utc()),
            status: Set(req.status),
            ..Default::default()
        }
    }

    fn update_model(req: Self::UpdatePayload, app: apps::Model) -> apps::ActiveModel {
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
        app
    }

    fn build_query(payload: Self::SearchPayLoad) -> sea_orm::Select<apps::Entity> {
        let mut query = apps::Entity::find()
            .filter(apps::Column::DeletedAt.is_null())
            .order_by_desc(apps::Column::CreatedAt);

        if let Some(name) = payload.name.filter(|n| !n.is_empty()) {
            query = query.filter(apps::Column::Name.contains(&name));
        }
        query
    }
}
