use crate::types::reg_codes_types::*;
crate::import_crud_macro!();
use entity::{app_devices, apps, reg_codes};
use salvo::{oapi::extract::JsonBody, prelude::*};
use salvo_oapi::extract::PathParam;

// Create RegCode
#[handler]
pub async fn add(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<RegCodeInfo>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let json = req.parse_json::<CreateRegCodeReq>().await?;
    let entity = add_impl(&state, json).await?;
    Ok(ApiResponse::success(entity))
}

pub async fn add_impl(state: &AppState, req: CreateRegCodeReq) -> Result<RegCodeInfo, AppError> {
    let active_model = reg_codes::ActiveModel {
        code: Set(req.code),
        app_id: Set(req.app_id),
        valid_days: Set(req.valid_days),
        max_devices: Set(req.max_devices),
        status: Set(i16::from(req.status)),
        code_type: Set(i16::from(req.code_type)),
        expire_time: Set(req.expire_time),
        total_count: Set(req.total_count),
        use_count: Set(0),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
        ..Default::default()
    };
    let entity = active_model.insert(&state.db).await?;

    // Fetch with app information for response
    let result = reg_codes::Entity::find_by_id(entity.id)
        .find_also_related(apps::Entity)
        .find_also_related(app_devices::Entity)
        .one(&state.db)
        .await?;

    match result {
        Some((reg_code, app, device)) => Ok(RegCodeInfo::try_from((reg_code, app, device))?),
        None => Err(AppError::not_found(
            "reg_codes".to_string(),
            Some(entity.id),
        )),
    }
}

// Update RegCode
#[handler]
pub async fn update(
    depot: &mut Depot,
    id: PathParam<i32>,
    req: JsonBody<UpdateRegCodeReq>,
) -> Result<ApiResponse<RegCodeInfo>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let reg_code = update_impl(&state, id.into_inner(), req.into_inner()).await?;
    Ok(ApiResponse::success(reg_code))
}

pub async fn update_impl(
    state: &AppState,
    id: i32,
    req: UpdateRegCodeReq,
) -> Result<RegCodeInfo, AppError> {
    let reg_code = reg_codes::Entity::find_by_id(id).one(&state.db).await?;
    let reg_code =
        reg_code.ok_or_else(|| AppError::not_found("reg_codes".to_string(), Some(id)))?;

    let mut reg_code: reg_codes::ActiveModel = reg_code.into_active_model();
    crate::update_field_if_some!(reg_code, code, req.code);
    crate::update_field_if_some!(reg_code, app_id, req.app_id);
    crate::update_field_if_some!(reg_code, valid_days, req.valid_days);
    crate::update_field_if_some!(reg_code, max_devices, req.max_devices);
    crate::update_field_if_some!(reg_code, status, req.status);
    crate::update_field_if_some!(reg_code, code_type, req.code_type.map(|v| i16::from(v)));
    crate::update_field_if_some!(reg_code, total_count, req.total_count, option);
    reg_code.updated_at = Set(Utc::now());

    let updated_reg_code = reg_code.update(&state.db).await?;

    // Fetch with app information for response
    let result = reg_codes::Entity::find_by_id(updated_reg_code.id)
        .find_also_related(apps::Entity)
        .find_also_related(app_devices::Entity)
        .one(&state.db)
        .await?;

    match result {
        Some((reg_code, app, device)) => Ok(RegCodeInfo::try_from((reg_code, app, device))?),
        None => Err(AppError::not_found(
            "reg_codes".to_string(),
            Some(updated_reg_code.id),
        )),
    }
}

// Delete RegCode
#[handler]
pub async fn delete(depot: &mut Depot, id: PathParam<i32>) -> Result<ApiResponse<()>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    delete_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(()))
}

pub async fn delete_impl(state: &AppState, id: i32) -> Result<(), AppError> {
    let reg_code = reg_codes::Entity::find_by_id(id).one(&state.db).await?;
    let reg_code =
        reg_code.ok_or_else(|| AppError::not_found("reg_codes".to_string(), Some(id)))?;
    reg_code.into_active_model().delete(&state.db).await?;
    Ok(())
}

// Get RegCodes List
#[handler]
pub async fn get_list(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<PagingResponse<RegCodeInfo>>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let params = req.parse_queries::<SearchRegCodesParams>()?;
    let list = get_list_impl(&state, params).await?;
    Ok(ApiResponse::success(list))
}

pub async fn get_list_impl(
    state: &AppState,
    params: SearchRegCodesParams,
) -> Result<PagingResponse<RegCodeInfo>, AppError> {
    let page = params.pagination.page.unwrap_or(1);
    let page_size = params.pagination.page_size.unwrap_or(20);

    let mut query = reg_codes::Entity::find()
        .find_also_related(apps::Entity)
        .find_also_related(app_devices::Entity)
        .order_by_desc(reg_codes::Column::CreatedAt);

    crate::filter_if_some!(query, reg_codes::Column::Id, params.id, eq);
    crate::filter_if_some!(query, reg_codes::Column::Code, params.code, contains);
    crate::filter_if_some!(query, reg_codes::Column::AppId, params.app_id, eq);
    crate::filter_if_some!(query, reg_codes::Column::Status, params.status, eq);
    crate::filter_if_some!(
        query,
        reg_codes::Column::CodeType,
        params.code_type.map(|v| i16::from(v)),
        eq
    );

    let paginator = query.paginate(&state.db, page_size);
    let total = paginator.num_items().await.unwrap_or(0);
    let results = paginator.fetch_page(page - 1).await?;

    let list: Result<Vec<RegCodeInfo>, AppError> = results
        .into_iter()
        .map(|(reg_code, app, device)| RegCodeInfo::try_from((reg_code, app, device)))
        .collect();

    Ok(PagingResponse {
        list: list?,
        total,
        page,
    })
}

// Get RegCode by ID
#[handler]
pub async fn get_by_id(
    depot: &mut Depot,
    id: PathParam<i32>,
) -> Result<ApiResponse<RegCodeInfo>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let reg_code = get_by_id_impl(&state, id.into_inner()).await?;
    Ok(ApiResponse::success(reg_code))
}

pub async fn get_by_id_impl(state: &AppState, id: i32) -> Result<RegCodeInfo, AppError> {
    let result = reg_codes::Entity::find_by_id(id)
        .find_also_related(apps::Entity)
        .find_also_related(app_devices::Entity)
        .one(&state.db)
        .await?;

    match result {
        Some((reg_code, app, device)) => Ok(RegCodeInfo::try_from((reg_code, app, device))?),
        None => Err(AppError::not_found("reg_codes".to_string(), Some(id))),
    }
}

/// Validate registration code for device
// #[handler]
// refer https://github.com/salvo-rs/salvo/blob/main/crates/oapi/docs/endpoint.md
#[endpoint(tags("reg_codes"))]
pub async fn validate_code(
    depot: &mut Depot,
    req: JsonBody<RegCodeValidateReq>,
) -> Result<ApiResponse<RegCodeValidateResp>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let resp = validate_code_impl(&state, req.into_inner()).await?;
    Ok(ApiResponse::success(resp))
}

/// Validate registration code for device (GET)
// #[handler]
#[endpoint(
    tags( "reg_codes" ),
    parameters(
        ("code"=Option<String>,Query, description = "注册码"),
    ("app_key"=String, Query, description = "应用校验Key"),
    ("device_id"=String, Query, description = "设备ID")
))]
pub async fn validate_code_get(
    depot: &mut Depot,
    req: &mut Request,
) -> Result<ApiResponse<RegCodeValidateResp>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let json = req.parse_queries::<RegCodeValidateReq>()?;
    let resp = validate_code_impl(&state, json).await?;
    Ok(ApiResponse::success(resp))
}

pub async fn validate_code_impl(
    state: &AppState,
    req: RegCodeValidateReq,
) -> Result<RegCodeValidateResp, AppError> {
    // find app by app_valid_key
    let app = apps::Entity::find()
        .filter(apps::Column::AppValidKey.eq(req.app_key.clone()))
        .one(&state.db)
        .await?;
    let app = app.ok_or(AppError::not_found("apps".to_string(), None))?;
    let now = chrono::Utc::now();
    // let app_expire = now + chrono::Duration::days(app.trial_days as i64);
    if req.code.is_none() {
        let dev = app_devices::Entity::find()
            .filter(
                app_devices::Column::AppId
                    .eq(app.id)
                    .and(app_devices::Column::DeviceId.eq(req.device_id.clone())),
            )
            .one(&state.db)
            .await?;
        let mut expire = now + chrono::Duration::days(app.trial_days as i64);
        if dev.is_none() {
            //bind device
            let _ = app_devices::ActiveModel {
                app_id: Set(app.id),
                device_id: Set(req.device_id.clone()),
                device_info: Set(None),
                bind_time: Set(Some(Utc::now())),
                expire_time: Set(Some(expire)),
                ..Default::default()
            }
            .insert(&state.db)
            .await?;
        } else {
            expire = dev.unwrap().expire_time.unwrap();
            if now > expire {
                return Err(AppError::Message("device expired".into()));
            }
        }
        return Ok(RegCodeValidateResp {
            code_type: CodeType::Time,
            expire_time: Some(expire),
            remaining_count: None,
        });
    }
    // find reg code
    let rc = reg_codes::Entity::find()
        .filter(
            reg_codes::Column::Code
                .eq(req.code.clone())
                .and(reg_codes::Column::AppId.eq(app.id)),
        )
        .one(&state.db)
        .await?;
    let rc = rc.ok_or(AppError::not_found("reg_code".to_string(), None))?;
    // logic by type
    let mut active = rc.clone().into_active_model();
    match rc.code_type.into() {
        CodeType::Time => {
            // time-based
            let expire = rc
                .expire_time
                .or_else(|| Some(now + chrono::Duration::days(rc.valid_days as i64)));
            if let Some(exp) = expire {
                if now > exp {
                    active.status = Set(RegCodeStatus::Expired.into());
                    active.update(&state.db).await?;
                    return Err(AppError::Message("code expired".into()));
                }
            }
            // bind device id
            if rc.device_id.is_none() {
                // create or find app_device
                let dev = app_devices::Entity::find()
                    .filter(
                        app_devices::Column::AppId
                            .eq(app.id)
                            .and(app_devices::Column::DeviceId.eq(req.device_id.clone())),
                    )
                    .one(&state.db)
                    .await?;
                let dev_id = if let Some(d) = dev {
                    d.id
                } else {
                    app_devices::ActiveModel {
                        app_id: Set(app.id),
                        device_id: Set(req.device_id.clone()),
                        device_info: Set(None),
                        bind_time: Set(Some(Utc::now())),
                        ..Default::default()
                    }
                    .insert(&state.db)
                    .await?
                    .id
                };
                active.device_id = Set(Some(dev_id));
                active.status = Set(RegCodeStatus::Used.into());
                active.binding_time = Set(Some(Utc::now()));
                active.expire_time = Set(expire);
                active.update(&state.db).await?;
            }
            Ok(RegCodeValidateResp {
                code_type: CodeType::Time,
                expire_time: expire,
                remaining_count: None,
            })
        }
        CodeType::Count => {
            // count-based
            let total = rc.total_count.unwrap_or(0);
            let used = rc.use_count;
            if used >= total {
                return Err(AppError::Message("code used up".into()));
            }
            active.use_count = Set(used + 1);
            if rc.device_id.is_none() {
                let dev = app_devices::Entity::find()
                    .filter(
                        app_devices::Column::AppId
                            .eq(app.id)
                            .and(app_devices::Column::DeviceId.eq(req.device_id.clone())),
                    )
                    .one(&state.db)
                    .await?;
                let dev_id = if let Some(d) = dev {
                    d.id
                } else {
                    app_devices::ActiveModel {
                        app_id: Set(app.id),
                        device_id: Set(req.device_id.clone()),
                        device_info: Set(None),
                        bind_time: Set(Some(Utc::now())),
                        ..Default::default()
                    }
                    .insert(&state.db)
                    .await?
                    .id
                };
                active.device_id = Set(Some(dev_id));
            }
            active.status = Set(1);
            active.update(&state.db).await?;
            Ok(RegCodeValidateResp {
                code_type: CodeType::Count,
                expire_time: None,
                remaining_count: Some(total - used - 1),
            })
        }
    }
}
