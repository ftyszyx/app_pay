use crate::types::invite_records_types::*;
crate::import_crud_macro!();
use entity::{invite_records, users};
use sea_orm::{
    FromQueryResult, JoinType, QuerySelect, RelationTrait, prelude::Expr, sea_query::Alias,
};

// Create InviteRecord
#[utoipa::path(
    post,
    path = "/api/admin/invite_records",
    security(("api_key" = [])),
    request_body = CreateInviteRecordReq,
    responses((status = 200, description = "Success", body = invite_records::Model))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<CreateInviteRecordReq>,
) -> Result<ApiResponse<invite_records::Model>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(
    state: &AppState,
    req: CreateInviteRecordReq,
) -> Result<invite_records::Model, AppError> {
    let active_model = invite_records::ActiveModel {
        user_id: Set(req.user_id),
        inviter_id: Set(req.inviter_id),
        user_info: Set(req.user_info),
        created_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update InviteRecord
#[utoipa::path(
    put,
    path = "/api/admin/invite_records/{id}",
    security(("api_key" = [])),
    request_body = UpdateInviteRecordReq,
    responses((status = 200, description = "Success", body = InviteRecordInfo))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateInviteRecordReq>,
) -> Result<ApiResponse<invite_records::Model>, AppError> {
    let record = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(record))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: UpdateInviteRecordReq,
) -> Result<invite_records::Model, AppError> {
    let record = invite_records::Entity::find_by_id(id)
        .one(&state.db)
        .await?;
    let record =
        record.ok_or_else(|| AppError::not_found("invite_records".to_string(), Some(id)))?;
    let mut record: invite_records::ActiveModel = record.into_active_model();
    crate::update_field_if_some!(record, user_id, req.user_id);
    crate::update_field_if_some!(record, inviter_id, req.inviter_id);
    crate::update_field_if_some!(record, user_info, req.user_info, option);
    let record = record.update(&state.db).await?;
    Ok(record)
}

// Delete InviteRecord
#[utoipa::path(
    delete,
    path = "/api/admin/invite_records/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = serde_json::Value))
)]
pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<()>, AppError> {
    delete_impl(&state, id).await?;
    Ok(ApiResponse::success(()))
}

pub async fn delete_impl(state: &AppState, id: i32) -> Result<(), AppError> {
    let record = invite_records::Entity::find_by_id(id)
        .one(&state.db)
        .await?;
    let record =
        record.ok_or_else(|| AppError::not_found("invite_records".to_string(), Some(id)))?;
    record.into_active_model().delete(&state.db).await?;
    Ok(())
}

// Get InviteRecords List
#[utoipa::path(
    get,
    path = "/api/admin/invite_records/list",
    security(("api_key" = [])),
    params(SearchInviteRecordsParams),
    responses((status = 200, description = "Success", body = PagingResponse<InviteRecordInfo>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<SearchInviteRecordsParams>,
) -> Result<ApiResponse<PagingResponse<InviteRecordInfo>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchInviteRecordsParams,
) -> Result<PagingResponse<InviteRecordInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);

    let inviter_alias = Alias::new("inviter");
    let user_alias = Alias::new("user");
    let mut query = invite_records::Entity::find()
        .join_as(
            JoinType::LeftJoin,
            invite_records::Relation::Users.def(),
            user_alias.clone(),
        )
        .join_as(
            JoinType::LeftJoin,
            invite_records::Relation::Inviters.def(),
            inviter_alias.clone(),
        )
        .select_only()
        .column(invite_records::Column::Id)
        .column(invite_records::Column::UserId)
        .column(invite_records::Column::InviterId)
        .column(invite_records::Column::UserInfo)
        .column(invite_records::Column::CreatedAt)
        .column(users::Column::Username)
        .column_as(
            Expr::col((inviter_alias, users::Column::Username)),
            "inviter_username",
        )
        .order_by_desc(invite_records::Column::CreatedAt);

    if let Some(id) = params.id {
        query = query.filter(invite_records::Column::Id.eq(id));
    }
    if let Some(user_id) = params.user_id {
        query = query.filter(invite_records::Column::UserId.eq(user_id));
    }
    if let Some(inviter_id) = params.inviter_id {
        query = query.filter(invite_records::Column::InviterId.eq(inviter_id));
    }

    let paginator = query
        .into_model::<InviteRecordInfo>()
        .paginate(&state.db, page_size);
    let total = paginator.num_items().await?;
    let list = paginator.fetch_page(page - 1).await?;

    Ok(PagingResponse { list, total, page })
}

// Get InviteRecord by ID
#[utoipa::path(
    get,
    path = "/api/admin/invite_records/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = InviteRecordInfo))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<InviteRecordInfo>, AppError> {
    let record = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(record))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<InviteRecordInfo, AppError> {
    let inviter_alias = Alias::new("inviter");

    let record = invite_records::Entity::find_by_id(id)
        .join(JoinType::LeftJoin, invite_records::Relation::Users.def())
        .join_as(
            JoinType::LeftJoin,
            invite_records::Relation::Users.def(),
            inviter_alias.clone(),
        )
        .select_only()
        .column(invite_records::Column::Id)
        .column(invite_records::Column::UserId)
        .column(invite_records::Column::InviterId)
        .column(invite_records::Column::UserInfo)
        .column(invite_records::Column::CreatedAt)
        .column(users::Column::Username)
        .column_as(
            Expr::col((inviter_alias, users::Column::Username)),
            "inviter_username",
        )
        .into_model::<InviteRecordInfo>()
        .one(&state.db)
        .await?;

    record.ok_or_else(|| AppError::not_found("invite_records".to_string(), Some(id)))
}
