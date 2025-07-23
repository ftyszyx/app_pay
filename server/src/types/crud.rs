use crate::types::error::AppError;
use validator::Validate;

pub trait CrudOperations: Sized {
    type Entity: sea_orm::EntityTrait;
    type ActiveModel: sea_orm::ActiveModelTrait;
    type Model: sea_orm::ModelTrait + std::fmt::Debug;
    type CreatePayload: serde::de::DeserializeOwned
        + utoipa::ToSchema
        + std::fmt::Debug
        + validator::Validate;
    type UpdatePayload: serde::de::DeserializeOwned
        + utoipa::ToSchema
        + std::fmt::Debug
        + validator::Validate;
    type SearchPayLoad: serde::de::DeserializeOwned + utoipa::ToSchema + std::fmt::Debug;
    type SearchResult: serde::de::DeserializeOwned + utoipa::ToSchema + std::fmt::Debug;
    type QueryResult: Send + Sync;

    fn table_name() -> &'static str;

    fn create_model(
        payload: Self::CreatePayload,
    ) -> <Self::Entity as sea_orm::EntityTrait>::ActiveModel;

    fn update_model(
        payload: Self::UpdatePayload,
        model: <Self::Entity as sea_orm::EntityTrait>::Model,
    ) -> <Self::Entity as sea_orm::EntityTrait>::ActiveModel;

    fn build_query(_: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        Err(AppError::Message("not implemented build_query".to_string()))
    }
    fn build_query_by_id(_id: i32) -> Result<Self::QueryResult, AppError> {
        Err(AppError::Message(
            "not implemented build_query_by_id".to_string(),
        ))
    }

    // 可选的钩子方法
    fn before_add(_payload: &Self::CreatePayload) -> Result<(), AppError> {
        _payload.validate()?;
        tracing::info!("before add : {} data: {:?}", Self::table_name(), _payload);
        Ok(())
    }

    fn after_add(_model: &Self::Model) -> Result<(), AppError> {
        tracing::info!("after add : {} data: {:?}", Self::table_name(), _model);
        Ok(())
    }

    fn before_update(_id: i32, _payload: &Self::UpdatePayload) -> Result<(), AppError> {
        _payload.validate()?;
        tracing::info!(
            "before update : {} data: {:?}",
            Self::table_name(),
            _payload
        );
        Ok(())
    }

    fn after_update(_model: &Self::Model) -> Result<(), AppError> {
        tracing::info!("after update : {} data: {:?}", Self::table_name(), _model);
        Ok(())
    }

    fn before_delete(_id: i32) -> Result<(), AppError> {
        tracing::info!("before delete : {} data: {}", Self::table_name(), _id);
        Ok(())
    }

    fn after_delete(_id: i32) -> Result<(), AppError> {
        tracing::info!("after delete : {} data: {}", Self::table_name(), _id);
        Ok(())
    }
}
