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
    apps::Model,
    "apps",
    true
);

impl CrudOperations for AppHandler {
    type Entity = apps::Entity;
    type CreatePayload = AddAppReq;
    type UpdatePayload = UpdateAppReq;
    type SearchPayLoad = ListAppsParams;
    type ActiveModel = apps::ActiveModel;
    type SearchResult = apps::Model;
    type Model = apps::Model;
    type QueryResult = sea_orm::Select<apps::Entity>;
    fn table_name() -> &'static str {
        "apps"
    }

    fn create_model(req: Self::CreatePayload) -> Result<Self::ActiveModel, AppError> {
        Ok(apps::ActiveModel {
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
        })
    }

    fn update_model(
        req: Self::UpdatePayload,
        app: apps::Model,
    ) -> Result<Self::ActiveModel, AppError> {
        let mut app: apps::ActiveModel = app.into_active_model();
        crate::update_field_if_some!(app, name, req.name);
        crate::update_field_if_some!(app, app_id, req.app_id);
        crate::update_field_if_some!(app, app_vername, req.app_vername);
        crate::update_field_if_some!(app, app_vercode, req.app_vercode);
        crate::update_field_if_some!(app, app_download_url, req.app_download_url);
        crate::update_field_if_some!(app, app_res_url, req.app_res_url);
        crate::update_field_if_some!(app, app_update_info, req.app_update_info, option);
        crate::update_field_if_some!(app, sort_order, req.sort_order);
        crate::update_field_if_some!(app, status, req.status);
        Ok(app)
    }

    fn build_query(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let mut query = apps::Entity::find()
            .filter(apps::Column::DeletedAt.is_null())
            .order_by_desc(apps::Column::CreatedAt);

        crate::filter_if_some!(query, apps::Column::Name, payload.name, contains);
        crate::filter_if_some!(query, apps::Column::Id, payload.id, eq);
        crate::filter_if_some!(query, apps::Column::AppId, payload.app_id, contains);
        Ok(query)
    }

    fn build_query_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::build_query(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
}
