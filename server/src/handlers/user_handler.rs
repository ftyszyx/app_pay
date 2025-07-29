use crate::types::user_types::*;
use entity::{roles, users};
use migration::SubQueryStatement;
use sea_orm::{ConnectionTrait, DatabaseBackend, JoinType, QuerySelect, Statement};
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
    let offset = (page - 1) * page_size;

    // 构建 WHERE 条件
    let mut where_conditions = vec!["u.deleted_at IS NULL".to_string()];
    let mut param_values: Vec<sea_orm::Value> = vec![];
    let mut param_index = 1;

    // 添加筛选条件
    if let Some(id) = params.id {
        where_conditions.push(format!("u.id = ${}", param_index));
        param_values.push(id.into());
        param_index += 1;
    }

    if let Some(username) = params.username {
        if !username.is_empty() {
            where_conditions.push(format!("u.username ILIKE ${}", param_index));
            param_values.push(format!("%{}%", username).into());
            param_index += 1;
        }
    }

    if let Some(user_id) = params.user_id {
        if !user_id.is_empty() {
            where_conditions.push(format!("u.user_id = ${}", param_index));
            param_values.push(user_id.into());
            param_index += 1;
        }
    }

    let where_clause = where_conditions.join(" AND ");

    // 查询数据的 SQL
    let data_sql = format!(r#"
        SELECT 
            u.id,
            u.username,
            u.balance,
            u.inviter_id,
            u.invite_rebate_total,
            u.role_id,
            r.name as role_name,
            COALESCE(COUNT(ir.id), 0) as invite_count,
            u.created_at
        FROM users u
        LEFT JOIN roles r ON u.role_id = r.id
        LEFT JOIN invite_records ir ON u.id = ir.inviter_id
        WHERE {}
        GROUP BY u.id, r.id
        ORDER BY u.id ASC
        LIMIT ${} OFFSET ${}
    "#, where_clause, param_index, param_index + 1);

    // 查询总数的 SQL
    let count_sql = format!(r#"
        SELECT COUNT(DISTINCT u.id) as total
        FROM users u
        LEFT JOIN roles r ON u.role_id = r.id
        LEFT JOIN invite_records ir ON u.id = ir.inviter_id
        WHERE {}
    "#, where_clause);

    // 添加分页参数
    param_values.push((page_size as i64).into());
    param_values.push((offset as i64).into());

    // 执行数据查询
    let users: Vec<UserInfo> = UserInfo::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            &data_sql,
            param_values.clone()
        )
    )
    .all(&state.db)
    .await?;

    // 执行总数查询（移除分页参数）
    let mut count_params = param_values;
    count_params.pop(); // 移除 offset
    count_params.pop(); // 移除 limit

    #[derive(FromQueryResult)]
    struct CountResult {
        total: i64,
    }

    let count_result: Option<CountResult> = CountResult::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            &count_sql,
            count_params
        )
    )
    .one(&state.db)
    .await?;

    let total = count_result.map(|c| c.total as u64).unwrap_or(0);

    Ok(PagingResponse {
        list: users,
        total,
        page,
    })
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
    let sql=r#"
    SELECT 
            u.id,
            u.username,
            u.balance,
            u.inviter_id,
            u.invite_rebate_total,
            u.role_id,
            r.name as role_name,
            COALESCE(COUNT(ir.id), 0) as invite_count,
            u.created_at
        FROM users u
        LEFT JOIN roles r ON u.role_id = r.id
        LEFT JOIN invite_records ir ON u.id = ir.inviter_id
        WHERE u.id = $1 AND u.deleted_at IS NULL
        GROUP BY u.id
    "#;
     let user_with_details: Option<UserInfo> = UserInfo::find_by_statement(
        Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [id.into()]
        )
    ).one(&state.db).await?;
    let user=user_with_details.ok_or_else(|| AppError::not_found("users".to_string(), Some(id)))?;
    Ok(user)
}

