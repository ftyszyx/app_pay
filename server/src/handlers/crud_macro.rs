#[macro_export]
macro_rules! impl_crud_handlers {
    (
        $prefix:ident,
        $entity:ty,
        $model:ty,
        $active_model:ty,
        $create_payload:ty,
        $update_payload:ty,
        $list_payload:ty,
        $create_logic:block,
        $update_logic:block,
        $list_filter_logic:block
    ) => {
        use axum::{extract::{Path, Query, State}, Json};
        use sea_orm::{
            ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, 
            IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, Set
        };
        use chrono::Utc;
        use serde_json::json;
        use crate::types::{common::*, response::ApiResponse};

        // 创建
        #[utoipa::path(
            post,
            path = concat!("/api/admin/", stringify!($prefix)),
            request_body = $create_payload,
            responses(
                (status = 200, description = "Created successfully", body = ApiResponse<$model>),
            )
        )]
        pub async fn add(
            State(db): State<DatabaseConnection>,
            Json(payload): Json<$create_payload>,
        ) -> Result<ApiResponse<$model>, AppError> {
            let new_entity: $active_model = $create_logic;
            let entity = new_entity.insert(&db).await?;
            Ok(ApiResponse::success(entity))
        }

        //update
        #[utoipa::path(
            put,
            path = concat!("/api/admin/", stringify!($prefix), "/{id}"),
            request_body = $update_payload,
            responses(
                (status = 200, description = "Updated successfully", body = ApiResponse<$model>),
            )
        )]
        pub async fn update(
            State(db): State<DatabaseConnection>,
            Json(req): Json<$update_payload>,
        ) -> Result<ApiResponse<$model>, AppError> {
            let entity = <$entity>::find_by_id(req.id).one(&db).await?;
            let entity = entity.ok_or(AppError::DataNotFound)?;
            let mut active_model = entity.into_active_model();
            $update_logic;
            let updated_entity = active_model.update(&db).await?;
            Ok(ApiResponse::success(updated_entity))
        }

        //delete
        #[utoipa::path(
            delete,
            path = concat!("/api/admin/", stringify!($prefix), "/{id}"),
            responses(
                (status = 200, description = "Deleted successfully", body = ApiResponse<serde_json::Value>),
            )
        )]
        pub async fn delete(
            State(db): State<DatabaseConnection>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<serde_json::Value>, AppError> {
            let entity = <$entity>::find_by_id(id).one(&db).await?;
            let entity = entity.ok_or(AppError::DataNotFound)?;
            let mut active_model = entity.into_active_model();
            active_model.deleted_at = Set(Some(Utc::now().naive_utc()));
            active_model.update(&db).await?;
            Ok(ApiResponse::success(json!({ "message": "success" })))
        }

        //list
        #[utoipa::path(
            post,
            path = concat!("/api/admin/", stringify!($prefix),"list"),
            responses(
                (status = 200, description = "List retrieved successfully", body = ApiResponse<PagingResponse<$model>>),
            )
        )]
        pub async fn get_list(
            State(db): State<DatabaseConnection>,
            Query(params): Query<ListParamsReq>,
            Json(payload): Json<$list_payload>,
        ) -> Result<ApiResponse<PagingResponse<$model>>, AppError> {
            let page = params.page;
            let page_size = params.page_size;
            let mut query = <$entity>::find()
                .filter(<$entity as EntityTrait>::Column::DeletedAt.is_null())
                .order_by_desc(<$entity as EntityTrait>::Column::CreatedAt);
            $list_filter_logic;
            let paginator = query.paginate(&db, page_size);
            let total = paginator.num_items().await.unwrap_or(0);
            let list = paginator.fetch_page(page - 1).await?;
            Ok(ApiResponse::success(PagingResponse {
                list,
                total,
                page,
            }))
        }

        //search
        #[utoipa::path(
            get,
            path = concat!("/api/admin/", stringify!($prefix), "/{id}"),
            responses(
                (status = 200, description = "Entity retrieved successfully", body = ApiResponse<$model>),
            )
        )]
        pub async fn get_by_id(
            State(db): State<DatabaseConnection>,
            Path(id): Path<i32>,
        ) -> Result<ApiResponse<$model>, AppError> {
            let entity = <$entity>::find_by_id(id)
                .filter(<$entity as EntityTrait>::Column::DeletedAt.is_null())
                .one(&db)
                .await?;
            let entity = entity.ok_or(AppError::DataNotFound)?;
            Ok(ApiResponse::success(entity))
        }
    };
} 