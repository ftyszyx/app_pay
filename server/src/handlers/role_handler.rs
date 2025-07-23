use crate::types::role_types::*;
use entity::roles;

crate::impl_crud_handlers!(
    RoleHandler,
    roles::Entity,
    roles::ActiveModel,
    roles::Model,
    RoleCreatePayload,
    RoleUpdatePayload,
    ListRolesParams,
    roles::Model,
    "roles",
    true
);

impl CrudOperations for RoleHandler {
    type Entity = roles::Entity;
    type ActiveModel = roles::ActiveModel;
    type Model = roles::Model;
    type CreatePayload = RoleCreatePayload;
    type UpdatePayload = RoleUpdatePayload;
    type SearchPayLoad = ListRolesParams;
    type SearchResult = roles::Model;
    type QueryResult = sea_orm::Select<roles::Entity>;
    fn table_name() -> &'static str {
        "roles"
    }

    fn create_model(payload: Self::CreatePayload) -> Result<Self::ActiveModel, AppError> {
        Ok(roles::ActiveModel {
            name: Set(payload.name),
            remark: Set(payload.remark),
            ..Default::default()
        })
    }

    fn update_model(
        payload: Self::UpdatePayload,
        role: Self::Model,
    ) -> Result<Self::ActiveModel, AppError> {
        let mut role: Self::ActiveModel = role.into_active_model();
        if let Some(name) = payload.name {
            role.name = Set(name);
        }
        Ok(role)
    }

    fn build_query(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let mut query = roles::Entity::find()
            .filter(roles::Column::DeletedAt.is_null())
            .order_by_asc(roles::Column::Id);
        crate::filter_if_some!(query, roles::Column::Name, payload.name, contains);
        crate::filter_if_some!(query, roles::Column::Id, payload.id, eq);
        Ok(query)
    }

    fn build_query_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::build_query(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
}
