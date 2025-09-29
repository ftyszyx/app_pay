//oss https://github.com/aliyun/alibabacloud-typescript-sdk/tree/master/sts-20150401
//https://github.com/aliyun/alibabacloud-typescript-sdk
//oss api
use crate::types::common::{AppState, Claims};
use crate::types::error::AppError;
use crate::types::response::ApiResponse;
use aliyun_sts::AssumeRoleRequest;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StsConfigReq {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StsCredentialsResp {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: String,
    pub expiration: String,
}

#[handler]
pub async fn get_oss_sts(
    depot: &mut Depot,
) -> Result<ApiResponse<StsCredentialsResp>, AppError> {
    let state = depot.obtain::<AppState>().unwrap();
    let _claims = depot.obtain::<Claims>().unwrap();
    let conf = state.config.clone();
    let sts = state.aliyun_sts.clone();
    let req = AssumeRoleRequest::new(
        &conf.oss.role_arn,
        &"salvo".to_string(),
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
