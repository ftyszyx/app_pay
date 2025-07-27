use crate::types::reg_codes_types::{CreateRegCodeReq, UpdateRegCodeReq, SearchRegCodesParams, RegCodeInfo};
use entity::reg_codes;
use entity::apps;

// RegCode Handler - 使用新的统一CRUD架构
crate::impl_crud_handlers!(
    RegCodeHandler,
    reg_codes::Entity,
    reg_codes::ActiveModel,
    reg_codes::Model,
    CreateRegCodeReq,
    UpdateRegCodeReq,
    SearchRegCodesParams,
    RegCodeInfo,
    "reg_codes",
    false
);

impl CrudOperations for RegCodeHandler {
    type Entity = reg_codes::Entity;
    type CreatePayload = CreateRegCodeReq;
    type UpdatePayload = UpdateRegCodeReq;
    type SearchPayLoad = SearchRegCodesParams;
    type SearchResult = RegCodeInfo;
    type ActiveModel = reg_codes::ActiveModel;
    type Model = reg_codes::Model;
    type QueryResult = sea_orm::SelectTwo<reg_codes::Entity, apps::Entity>;

    fn table_name() -> &'static str {
        "reg_codes"
    }

    fn create_model(payload: Self::CreatePayload) -> Result<Self::ActiveModel, AppError> {
        Ok(reg_codes::ActiveModel {
            code: Set(payload.code),
            app_id: Set(payload.app_id),
            bind_device_info: Set(payload.bind_device_info),
            valid_days: Set(payload.valid_days),
            max_devices: Set(payload.max_devices),
            status: Set(payload.status),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
    }

    fn update_model(
        payload: Self::UpdatePayload,
        reg_code: reg_codes::Model,
    ) -> Result<Self::ActiveModel, AppError> {
        let mut reg_code: reg_codes::ActiveModel = reg_code.into_active_model();
        crate::update_field_if_some!(reg_code, code, payload.code);
        crate::update_field_if_some!(reg_code, app_id, payload.app_id);
        crate::update_field_if_some!(reg_code, bind_device_info, payload.bind_device_info, option);
        crate::update_field_if_some!(reg_code, valid_days, payload.valid_days);
        crate::update_field_if_some!(reg_code, max_devices, payload.max_devices);
        crate::update_field_if_some!(reg_code, status, payload.status);
        crate::update_field_if_some!(reg_code, binding_time, payload.binding_time, option);
        reg_code.updated_at = Set(Utc::now());
        Ok(reg_code)
    }

    fn get_list(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let mut query = reg_codes::Entity::find()
            .find_also_related(apps::Entity)
            .order_by_desc(reg_codes::Column::CreatedAt);

        crate::filter_if_some!(query, reg_codes::Column::Id, payload.id, eq);
        crate::filter_if_some!(query, reg_codes::Column::Code, payload.code, contains);
        crate::filter_if_some!(query, reg_codes::Column::AppId, payload.app_id, eq);
        crate::filter_if_some!(query, reg_codes::Column::Status, payload.status, eq);
        
        Ok(query)
    }

    fn get_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::get_list(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
} 