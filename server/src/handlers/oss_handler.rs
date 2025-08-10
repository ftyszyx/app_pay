use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::types::common::AppState;
use crate::types::response::ApiResponse;
use crate::types::error::AppError;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize,ToSchema)]
pub struct StsConfigReq {
    // pub role_arn: String,
    // pub duration_seconds: Option<u32>,
    // pub session_name: Option<String>,
    // pub policy: Option<String>,
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
) -> Result<ApiResponse<StsCredentialsResp>, AppError> {
    let conf = &state.config.oss;
    let access_key_id = conf
        .access_key_id
        .as_ref()
        .ok_or_else(|| AppError::Message("OSS_ACCESS_KEY_ID not set".into()))?;
    let access_key_secret = conf
        .access_key_secret
        .as_ref()
        .ok_or_else(|| AppError::Message("OSS_ACCESS_KEY_SECRET not set".into()))?;

    // STS 入口
    let endpoint = conf
        .endpoint
        .clone()
        .unwrap_or_else(|| "sts.aliyuncs.com".to_string());



    // 参考阿里云 STS OpenAPI: AssumeRole
    // 这里为了简化，使用 GET + query 方式
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
    let mut params = vec![
        ("Action", "AssumeRole"),
        ("Format", "JSON"),
        ("Version", "2015-04-01"),
        ("AccessKeyId", access_key_id.as_str()),
        ("SignatureMethod", "HMAC-SHA1"),
        ("SignatureVersion", "1.0"),
        ("SignatureNonce", uuid::Uuid::new_v4().to_string().as_str()),
        ("Timestamp", &chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
        ("RoleArn", conf.role_arn.as_str()),
        ("RoleSessionName", role_session_name.as_str()),
        ("DurationSeconds", conf.sts_expire_secs.as_str()),
    ];
    if let Some(policy) = &req.policy {
        params.push(("Policy", policy));
    }

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


