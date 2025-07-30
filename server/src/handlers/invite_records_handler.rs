use crate::types::invite_records_types::*;
crate::import_crud_macro!();
use entity::{invite_records, users};
use sea_orm::{
    Select,
     prelude::Expr, sea_query::Alias, JoinType, QuerySelect,  RelationTrait
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
        inviter_user_id: Set(req.inviter_user_id),
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
    crate::update_field_if_some!(record, inviter_user_id, req.inviter_user_id);
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

pub  fn get_query()->Select<entity::invite_records::Entity>{
    let inviter_alias = Alias::new("inviter");
    let user_alias = Alias::new("user");
    let  query = invite_records::Entity::find()
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
        .column(invite_records::Column::InviterUserId)
        .column(invite_records::Column::UserInfo)
        .column(invite_records::Column::CreatedAt)
        .column_as(Expr::col((user_alias, users::Column::Username)), "user_username")
        .column_as(
            Expr::col((inviter_alias, users::Column::Username)),
            "inviter_username",
        );
    query
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchInviteRecordsParams,
) -> Result<PagingResponse<InviteRecordInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = get_query()
        .order_by_desc(invite_records::Column::CreatedAt);
    crate::filter_if_some!(query, invite_records::Column::Id, params.id, eq);
    crate::filter_if_some!(query, invite_records::Column::UserId, params.user_id, eq);
    crate::filter_if_some!(query, invite_records::Column::InviterUserId, params.inviter_id, eq);
    // println!("get query2:{}", query.build(sea_orm::DatabaseBackend::Postgres).to_string());
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
    let record = get_query()
        .filter(invite_records::Column::Id.eq(id))
        .into_model::<InviteRecordInfo>()
        .one(&state.db)
        .await?;
    record.ok_or_else(|| AppError::not_found("invite_records".to_string(), Some(id)))
}
