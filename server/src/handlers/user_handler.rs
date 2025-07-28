use crate::types::user_types::*;
use entity::{roles, users};
use migration::SubQueryStatement;
use sea_orm::QuerySelect;
use sea_orm::sea_query::{Expr, Query as SeaQuery, SimpleExpr};
use sea_orm::{FromQueryResult, RelationTrait};
use uuid::Uuid;
crate::import_crud_macro!();

// Custom struct to capture query results with calculated columns
#[derive(FromQueryResult)]
struct UserWithInviteCount {
    // User fields
    pub id: i32,
    pub user_id: String,
    pub username: String,
    pub password: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub balance: i64,
    pub inviter_id: Option<i32>,
    pub invite_rebate_total: i64,
    pub role_id: i32,
    // Role fields (optional because of left join)
    pub role_name: Option<String>,
    pub role_remark: Option<String>,
    // Calculated field
    pub invite_count: Option<i64>,
}

// Create User
#[utoipa::path(
    post,
    path = "/api/admin/users",
    security(("api_key" = [])),
    request_body = UserCreatePayload,
    responses((status = 200, description = "Success", body = users::Model))
)]
pub async fn add(
    State(state): State<AppState>,
    Json(req): Json<UserCreatePayload>,
) -> Result<ApiResponse<users::Model>, AppError> {
    let entity = add_impl(&state, req).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: UserCreatePayload) -> Result<users::Model, AppError> {
    let password = bcrypt::hash(req.password, 10)?;
    let active_model = users::ActiveModel {
        username: Set(req.username),
        password: Set(password),
        role_id: Set(req.role_id.unwrap_or(crate::constants::DEFAULT_ROLE_ID)),
        user_id: Set(Uuid::new_v4().to_string()),
        created_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update User
#[utoipa::path(
    put,
    path = "/api/admin/users/{id}",
    security(("api_key" = [])),
    request_body = UserUpdatePayload,
    responses((status = 200, description = "Success", body = users::Model))
)]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UserUpdatePayload>,
) -> Result<ApiResponse<users::Model>, AppError> {
    let user = update_impl(&state, id, req).await?;
    Ok(ApiResponse::success(user))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: UserUpdatePayload,
) -> Result<users::Model, AppError> {
    let user = users::Entity::find_by_id(id).one(&state.db).await?;
    let user = user.ok_or_else(|| AppError::not_found("users".to_string(), Some(id)))?;
    let mut user: users::ActiveModel = user.into_active_model();
    crate::update_field_if_some!(user, username, req.username);
    if let Some(password) = req.password {
        let hashed_password = bcrypt::hash(password, 10)?;
        user.password = Set(hashed_password);
    }
    crate::update_field_if_some!(user, role_id, req.role_id);
    crate::update_field_if_some!(user, balance, req.balance);
    let user = user.update(&state.db).await?;
    Ok(user)
}

// Delete User
#[utoipa::path(
    delete,
    path = "/api/admin/users/{id}",
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
    let user = users::Entity::find_by_id(id).one(&state.db).await?;
    let user = user.ok_or_else(|| AppError::not_found("users".to_string(), Some(id)))?;
    let mut user = user.into_active_model();
    user.deleted_at = Set(Some(Utc::now()));
    let _ = user.update(&state.db).await?;
    Ok(())
}

// Get Users List
#[utoipa::path(
    get,
    path = "/api/admin/users/list",
    security(("api_key" = [])),
    params(SearchUsersParams),
    responses((status = 200, description = "Success", body = PagingResponse<UserInfo>))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(params): Query<SearchUsersParams>,
) -> Result<ApiResponse<PagingResponse<UserInfo>>, AppError> {
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchUsersParams,
) -> Result<PagingResponse<UserInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);

    let subquery = SeaQuery::select()
        .expr(Expr::col(entity::invite_records::Column::Id).count())
        .from(entity::invite_records::Entity)
        .and_where(
            Expr::col(entity::invite_records::Column::InviterId)
                .eq(Expr::col(entity::users::Column::Id)),
        )
        .to_owned();

    let mut query = users::Entity::find()
        .select_only()
        // Select user columns
        .column_as(users::Column::Id, "id")
        .column_as(users::Column::UserId, "user_id")
        .column_as(users::Column::Username, "username")
        .column_as(users::Column::Password, "password")
        .column_as(users::Column::CreatedAt, "created_at")
        .column_as(users::Column::DeletedAt, "deleted_at")
        .column_as(users::Column::Balance, "balance")
        .column_as(users::Column::InviterId, "inviter_id")
        .column_as(users::Column::InviteRebateTotal, "invite_rebate_total")
        .column_as(users::Column::RoleId, "role_id")
        // Select role columns
        .column_as(roles::Column::Name, "role_name")
        .column_as(roles::Column::Remark, "role_remark")
        // Add calculated column
        .column_as(
            SimpleExpr::SubQuery(None, Box::new(SubQueryStatement::SelectStatement(subquery))),
            "invite_count",
        )
        .join(sea_orm::JoinType::LeftJoin, users::Relation::Roles.def())
        .filter(users::Column::DeletedAt.is_null())
        .order_by_desc(users::Column::CreatedAt);

    crate::filter_if_some!(query, users::Column::Id, params.id, eq);
    crate::filter_if_some!(query, users::Column::Username, params.username, contains);
    crate::filter_if_some!(query, users::Column::UserId, params.user_id, eq);

    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let results: Vec<UserWithInviteCount> = paginator
        .fetch_page(page - 1)
        .await?
        .into_iter()
        .map(|model| UserWithInviteCount::from_query_result(&model, ""))
        .collect::<Result<Vec<_>, _>>()?;

    let list: Vec<UserInfo> = results
        .into_iter()
        .map(|user_with_count| create_user_info_from_result(user_with_count))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PagingResponse { list, total, page })
}

// Get User by ID
#[utoipa::path(
    get,
    path = "/api/admin/users/{id}",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = UserInfo))
)]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<ApiResponse<UserInfo>, AppError> {
    let user = get_by_id_impl(&state, id).await?;
    Ok(ApiResponse::success(user))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<UserInfo, AppError> {
    let subquery = SeaQuery::select()
        .expr(Expr::col(entity::invite_records::Column::Id).count())
        .from(entity::invite_records::Entity)
        .and_where(
            Expr::col(entity::invite_records::Column::InviterId)
                .eq(Expr::col(entity::users::Column::Id)),
        )
        .to_owned();

    let result: Option<UserWithInviteCount> = users::Entity::find_by_id(id)
        .select_only()
        // Select user columns
        .column_as(users::Column::Id, "id")
        .column_as(users::Column::UserId, "user_id")
        .column_as(users::Column::Username, "username")
        .column_as(users::Column::Password, "password")
        .column_as(users::Column::CreatedAt, "created_at")
        .column_as(users::Column::DeletedAt, "deleted_at")
        .column_as(users::Column::Balance, "balance")
        .column_as(users::Column::InviterId, "inviter_id")
        .column_as(users::Column::InviteRebateTotal, "invite_rebate_total")
        .column_as(users::Column::RoleId, "role_id")
        // Select role columns
        .column_as(roles::Column::Name, "role_name")
        .column_as(roles::Column::Remark, "role_remark")
        // Add calculated column
        .column_as(
            SimpleExpr::SubQuery(None, Box::new(SubQueryStatement::SelectStatement(subquery))),
            "invite_count",
        )
        .join(sea_orm::JoinType::LeftJoin, users::Relation::Roles.def())
        .into_model::<UserWithInviteCount>()
        .one(&state.db)
        .await?;

    match result {
        Some(user_with_count) => {
            let user_info = create_user_info_from_result(user_with_count)?;
            Ok(user_info)
        }
        None => Err(AppError::not_found("users".to_string(), Some(id))),
    }
}

// Helper function to create UserInfo from UserWithInviteCount
fn create_user_info_from_result(user_result: UserWithInviteCount) -> Result<UserInfo, AppError> {
    let role_name = user_result
        .role_name
        .ok_or_else(|| AppError::Message("role not found".to_string()))?;
    Ok(UserInfo {
        id: user_result.id,
        username: user_result.username,
        balance: user_result.balance.to_string(),
        inviter_id: user_result.inviter_id,
        invite_count: user_result.invite_count.unwrap_or(0) as u64,
        invite_rebate_total: user_result.invite_rebate_total,
        role_id: user_result.role_id,
        role_name,
        created_at: user_result
            .created_at
            .format("%Y-%m-%d %H:%M:%S")
            .to_string(),
    })
}
