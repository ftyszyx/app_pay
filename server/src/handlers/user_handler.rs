use crate::types::common::{AppState, PagingResponse};
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use crate::types::user_types::*;
use salvo::{prelude::*, oapi::extract::JsonBody};
use salvo_oapi::extract::{PathParam};
use chrono::Utc;
use entity::{ roles, users};
use migration::{Alias, Expr};
use sea_orm::{ QueryFilter,JoinType,PaginatorTrait, Select,RelationTrait, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QuerySelect,  Set};

// Create User
#[handler]
pub async fn add(
    depot: &mut Depot,
    req: JsonBody<UserCreatePayload>,
) -> Result<ApiResponse<users::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let entity = add_impl(&state, req.into_inner()).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: UserCreatePayload) -> Result<users::Model, AppError> {
    let password = bcrypt::hash(req.password, 10)?;
    let active_model = users::ActiveModel {
        username: Set(req.username),
        password: Set(password),
        role_id: Set(req.role_id.unwrap_or(crate::constants::DEFAULT_ROLE_ID)),
        created_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;
    Ok(entity)
}

// Update User
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: PathParam<i32>,
    req: JsonBody<UserUpdatePayload>,
) -> Result<ApiResponse<users::Model>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let user = update_impl(&state, id.into_inner(), req.into_inner()).await?;
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
#[handler]
pub async fn delete(
    depot: &mut Depot,
    id: PathParam<i32>,
) -> Result<ApiResponse<()>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    delete_impl(&state, id.into_inner()).await?;
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
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<PagingResponse<UserInfo>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let params = req.parse_queries::<SearchUsersParams>()?;
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub fn get_query()->Select<entity::users::Entity>{
    let role_alias=Alias::new("role");
    //get invite num for userid
    let query=users::Entity::find()
        .join_as(JoinType::LeftJoin, users::Relation::Roles.def(), role_alias.clone())
    .select_only()
    .column_as(users::Column::Id, "id")
    .column_as(users::Column::Username, "username")
    .column_as(users::Column::Balance, "balance")
    .column_as(users::Column::RoleId, "role_id")
    .column_as(Expr::cust(
        "(SELECT COUNT(*) FROM invite_records WHERE inviter_user_id = users.id)",
    ), "invite_count")
    .column_as(Expr::col((role_alias, roles::Column::Name)), "role_name")
    .column_as(users::Column::InviteRebateTotal, "invite_rebate_total")
    .column_as(users::Column::CreatedAt, "created_at");
    query
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchUsersParams,
) -> Result<PagingResponse<UserInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query=get_query();
    crate::filter_if_some!(query, users::Column::Id, params.id, eq);
    crate::filter_if_some!(query, users::Column::Username, params.username, like);
    let paginator=query.into_model::<UserInfo>().paginate(&state.db, page_size);
    let total=paginator.num_items().await?;
    let list=paginator.fetch_page(page-1).await?;
    Ok(PagingResponse { list, total, page })
}

// Get User by ID
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: PathParam<i32>,
) -> Result<ApiResponse<UserInfo>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let user = get_by_id_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(user))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<UserInfo, AppError> {
    let query=get_query();
    let query=query.filter(users::Column::Id.eq(id));
    let result: Option<UserInfo> =query.into_model::<UserInfo>().one(&state.db).await?;
    let user = result.ok_or_else(|| AppError::not_found("users".to_string(), Some(id)))?;
    return Ok(user);
}
