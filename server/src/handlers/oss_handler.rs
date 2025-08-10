//oss api
//https://help.aliyun.com/zh/oss/developer-reference/api-reference/?spm=a2c4g.11186623.help-menu-31815.d_19_1.691a4425Xf61lE&scm=20140722.H_31946._.OR_help-T_cn~zh-V_1
//https://help.aliyun.com/zh/oss/?spm=a2c4g.11186623.0.0.723cb930I5VSPy

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::types::common::{AppState, Claims};
use crate::types::response::ApiResponse;
use crate::types::error::AppError;
use utoipa::ToSchema;
use axum::Extension;
use crate::types::config::Config;

#[derive(Debug, Serialize, Deserialize,ToSchema)]
pub struct StsConfigReq {
    // pub role_arn: String,
    // pub duration_seconds: Option<u32>,
    // pub session_name: Option<String>,
    // pub policy: Option<String>,
}

fn get_oss_commom_params(conf: &Config) -> Vec<(&str, String)> {
    vec![
        ("Format", "JSON".to_string()),
        ("Version", "2015-04-01".to_string()),
        ("AccessKeyId", conf.oss.access_key_id.clone()),
        ("SignatureMethod", "HMAC-SHA1".to_string()),
        ("SignatureVersion", "1.0".to_string()),
        ("SignatureNonce", uuid::Uuid::new_v4().to_string()),
        ("Timestamp", chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
    ]
}

#[derive(Debug, Serialize, Deserialize,ToSchema)]
pub struct StsCredentialsResp {
    pub access_key_id: String,
    pub access_key_secret: String,
    pub security_token: String,
    pub expiration: u32,
}

#[utoipa::path(
    post,
    path = "/api/admin/storage/oss/sts",
    security(("api_key" = [])),
    request_body = StsConfigReq,
    responses((status = 200, description = "Success", body = StsCredentialsResp))
)]
pub async fn get_oss_sts(
    State(state): State<AppState>,
    Json(req): Json<StsConfigReq>,
        Extension(claims): Extension<Claims>,
) -> Result<ApiResponse<StsCredentialsResp>, AppError> {

    // 参考阿里云 STS OpenAPI: AssumeRole
    // 这里为了简化，使用 GET + query 方式
    let conf=state.config;
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let params = get_oss_commom_params(&conf);
    params.push(("Action", "AssumeRole".to_string()));
    params.push(("RoleArn", conf.oss.role_arn.clone()));
    params.push(("RoleSessionName", claims.sub.to_string()));
    params.push(("DurationSeconds", conf.oss.sts_expire_secs.to_string()));


    // 构造待签名串
    let mut pairs: Vec<(String, String)> = params
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                utf8_percent_encode(v, NON_ALPHANUMERIC).to_string(),
            )
        })
        .collect();
    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    let canonicalized = pairs
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");
    let string_to_sign = format!("GET&%2F&{}", utf8_percent_encode(&canonicalized, NON_ALPHANUMERIC));
    let sign_key = format!("{}&", access_key_secret);
    let signature = base64::engine::general_purpose::STANDARD.encode(hmac_sha1::hmac_sha1(sign_key.as_bytes(), string_to_sign.as_bytes()));
    let signature_enc = utf8_percent_encode(&signature, NON_ALPHANUMERIC).to_string();

    // 发送请求
    let url = format!(
        "https://{}?{}&Signature={}",
        endpoint,
        canonicalized,
        signature_enc
    );
    let resp = reqwest::get(&url).await.map_err(|e| AppError::Message(e.to_string()))?;
    if !resp.status().is_success() {
        return Err(AppError::Message(format!("STS request failed: {}", resp.status())));
    }
    #[derive(Deserialize)]
    struct AssumeRoleBody {
        #[serde(rename = "Credentials")]
        credentials: Credentials,
    }
    #[derive(Deserialize)]
    struct Credentials {
        #[allow(non_snake_case)]
        AccessKeyId: String,
        #[allow(non_snake_case)]
        AccessKeySecret: String,
        #[allow(non_snake_case)]
        SecurityToken: String,
        #[allow(non_snake_case)]
        Expiration: String,
    }
    #[derive(Deserialize)]
    struct AssumeRoleResp { AssumeRoleResponse: AssumeRoleBody }
    let body: AssumeRoleResp = resp.json().await.map_err(|e| AppError::Message(e.to_string()))?;
    let c = body.AssumeRoleResponse.credentials;
    Ok(ApiResponse::success(StsCredentialsResp {
        access_key_id: c.AccessKeyId,
        access_key_secret: c.AccessKeySecret,
        security_token: c.SecurityToken,
        expiration: c.Expiration,
    }))
}


