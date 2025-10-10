use entity::{app_devices, apps};
use salvo::{prelude::*};
use crate::types::app_devices_types::*;
use crate::types::common::*;
use crate::types::error::*;
use crate::types::response::*;
use sea_orm::{
    ColumnTrait, EntityTrait,  PaginatorTrait, QueryFilter,
    QueryOrder,
};

#[handler]
pub async fn get_list(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<PagingResponse<DeviceInfo>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let params = req.parse_queries::<SearchDevicesParams>()?;
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchDevicesParams,
) -> Result<PagingResponse<DeviceInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);
    let mut query = app_devices::Entity::find()
        .find_also_related(apps::Entity)
        .order_by_desc(app_devices::Column::BindTime);
    crate::filter_if_some!(query, app_devices::Column::AppId, params.app_id, eq);
    crate::filter_if_some!(query, app_devices::Column::DeviceId, params.device_id, eq);
    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let result= paginator.fetch_page(page - 1).await?;
    let list = result
        .into_iter()
        .filter_map(|item| DeviceInfo::try_from(item).ok())
        .collect();
    Ok(PagingResponse { list, total, page })
}
