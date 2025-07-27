use crate::types::invite_records_types::{
    CreateInviteRecordReq, InviteRecordInfo, SearchInviteRecordsParams, UpdateInviteRecordReq,
};
use entity::invite_records;
use entity::users;

// InviteRecord Handler - 使用新的统一CRUD架构
crate::impl_crud_handlers!(
    InviteRecordHandler,
    invite_records::Entity,
    invite_records::ActiveModel,
    invite_records::Model,
    CreateInviteRecordReq,
    UpdateInviteRecordReq,
    SearchInviteRecordsParams,
    InviteRecordInfo,
    "invite_records",
    false
);

impl CrudOperations for InviteRecordHandler {
    type Entity = invite_records::Entity;
    type CreatePayload = CreateInviteRecordReq;
    type UpdatePayload = UpdateInviteRecordReq;
    type SearchPayLoad = SearchInviteRecordsParams;
    type SearchResult = InviteRecordInfo;
    type ActiveModel = invite_records::ActiveModel;
    type Model = invite_records::Model;
    type QueryResult = sea_orm::SelectTwo<invite_records::Entity>;

    fn table_name() -> &'static str {
        "invite_records"
    }

    fn create_model(payload: Self::CreatePayload) -> Result<Self::ActiveModel, AppError> {
        Ok(invite_records::ActiveModel {
            user_id: Set(payload.user_id),
            inviter_id: Set(payload.inviter_id),
            user_info: Set(payload.user_info),
            created_at: Set(Utc::now()),
            ..Default::default()
        })
    }

    fn update_model(
        payload: Self::UpdatePayload,
        record: invite_records::Model,
    ) -> Result<Self::ActiveModel, AppError> {
        let mut record: invite_records::ActiveModel = record.into_active_model();
        crate::update_field_if_some!(record, user_id, payload.user_id);
        crate::update_field_if_some!(record, inviter_id, payload.inviter_id);
        crate::update_field_if_some!(record, user_info, payload.user_info, option);
        Ok(record)
    }

    fn get_list(payload: Self::SearchPayLoad) -> Result<Self::QueryResult, AppError> {
        let subquery=SeaQuery::select()
            .expr(Expr::col(entity::users::Column::Username))
            .from(entity::users::Entity)
            .and_where(
                Expr::col(entity::invite_records::Column::InviterId)
                    .eq(Expr::col(entity::users::Column::Id)),
            )
            .to_owned();
        let subquery2=SeaQuery::select()
            .expr(Expr::col(entity::users::Column::Username))
            .from(entity::users::Entity)
            .and_where(
                Expr::col(entity::invite_records::Column::InviterId)
                    .eq(Expr::col(entity::users::Column::Id)),
            )
            .to_owned();
        let mut query = invite_records::Entity::find()
            .find_also_related(users::Entity)
            .find_also_related(users::Entity)
            .order_by_desc(invite_records::Column::CreatedAt);

        crate::filter_if_some!(query, invite_records::Column::Id, payload.id, eq);
        crate::filter_if_some!(query, invite_records::Column::UserId, payload.user_id, eq);
        crate::filter_if_some!(
            query,
            invite_records::Column::InviterId,
            payload.inviter_id,
            eq
        );

        Ok(query)
    }

    fn get_by_id(id: i32) -> Result<Self::QueryResult, AppError> {
        Self::get_list(Self::SearchPayLoad {
            id: Some(id),
            ..Default::default()
        })
    }
}
