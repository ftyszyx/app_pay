// 统一的CRUD宏 - 完整实现
#[macro_export]
macro_rules! import_crud_macro {
    () => {
        use crate::types::common::{AppState, PagingResponse};
        use crate::types::error::AppError;
        use crate::types::response::ApiResponse;
        use axum::{
            Json,
            extract::{Path, Query, State},
        };
        use chrono::Utc;
        use sea_orm::{
            ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait,
            QueryFilter, QueryOrder, Set,
        };
    };
}
#[macro_export]
macro_rules! filter_if_some {
    // 匹配 .eq() 这种单参数的 filter 方法
    ($query:expr, $column:expr, $value:expr, eq) => {
        if let Some(val) = $value {
            $query = $query.filter($column.eq(val));
        }
    };
    // 匹配 .contains() 这种需要引用的 filter 方法
    ($query:expr, $column:expr, $value:expr, contains) => {
        if let Some(val) = $value.filter(|s| !s.is_empty()) {
            $query = $query.filter($column.contains(&val));
        }
    };
    // 你可以根据需要添加更多模式，例如 `gt`, `lt` 等
    ($query:expr, $column:expr, $value:expr, gt) => {
        if let Some(val) = $value {
            $query = $query.filter($column.gt(val));
        }
    };
    //lt
    ($query:expr, $column:expr, $value:expr, lt) => {
        if let Some(val) = $value {
            $query = $query.filter($column.lt(val));
        }
    };
    //gt
    //like
    ($query:expr, $column:expr, $value:expr, like) => {
        if let Some(val) = $value.filter(|s| !s.is_empty()) {
            $query = $query.filter($column.like(&format!("%{}%", val)));
        }
    };
}

#[macro_export]
macro_rules! update_field_if_some {
    // 普通赋值
    ($model:expr, $field:ident, $value:expr) => {
        if let Some(val) = $value {
            $model.$field = Set(val);
        }
    };
    // -> 新增：处理 Option<T> 字段
    ($model:expr, $field:ident, $value:expr, option) => {
        if let Some(val) = $value {
            $model.$field = Set(Some(val));
        }
    };
    // 需要特殊处理的赋值（例如哈希密码）
    ($model:expr, $field:ident, $value:expr, with $handler:expr) => {
        if let Some(val) = $value {
            $model.$field = Set($handler(val));
        }
    };
}
