//oss api
use crate::types::common::{AppState, Claims};
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use aliyun_sts::AssumeRoleRequest;
use axum::Extension;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StsConfigReq {}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StsCredentialsResp {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: String,
    pub expiration: String,
}

#[utoipa::path(
    get,
    path = "/api/admin/storage/oss/sts",
    security(("api_key" = [])),
    responses((status = 200, description = "Success", body = StsCredentialsResp))
)]
pub async fn get_oss_sts(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<StsCredentialsResp>, AppError> {
    let conf = state.config;
    let sts = state.aliyun_sts;
    let req = AssumeRoleRequest::new(
        &conf.oss.role_arn,
        &claims.sub.to_string(),
        None,
        conf.oss.sts_expire_secs,
    );
    let resp = sts
        .assume_role(req)
        .await
        .map_err(|e| AppError::Message(e.to_string()))?;
    let credentials = resp
        .credentials
        .ok_or(AppError::Message("credentials is none".to_string()))?;
    Ok(ApiResponse::success(StsCredentialsResp {
        access_key_id: credentials.access_key_id,
        access_key_secret: credentials.access_key_secret,
        security_token: credentials.security_token,
        expiration: credentials.expiration,
    }))
}
