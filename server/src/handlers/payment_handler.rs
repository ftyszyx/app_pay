use crate::types::error::AppError;
use crate::types::{common::AppState, pay_types::*, response::ApiResponse};
use axum::{
    Json,
    extract::{Path, State},
};
use pay::{Payment, WechatConfig};

/// 创建支付宝订单
#[utoipa::path(
    post,
    path = "/api/payment/alipay/create",
    request_body = CreateAlipayOrderReq,
    responses(
        (status = 200, description = "Success", body = PaymentOrderResponse)
    )
)]
pub async fn create_alipay_order(
    State(state): State<AppState>,
    Json(req): Json<CreateAlipayOrderReq>,
) -> Result<ApiResponse<PaymentOrderResponse>, AppError> {
    // 从配置或数据库获取支付宝配置
    let config = get_alipay_config(&state).await?;
    let payment = Payment::new(config);

    let method = match req.payment_method.as_str() {
        "app" => "alipay.trade.app.pay",
        "web" => "alipay.trade.page.pay",
        "qr" => "alipay.trade.precreate",
        _ => {
            return Err(AppError::InternalError {
                message: "不支持的支付方式".to_string(),
            });
        }
    };

    let order_data = pay::alipay::prelude::ReqOrderBody {
        out_trade_no: req.out_trade_no.clone(),
        total_amount: req.total_amount,
        subject: req.subject,
        ..Default::default()
    };

    let result = payment
        .create_order(method, order_data)
        .await
        .map_err(|e| AppError::InternalError {
            message: format!("支付宝订单创建失败: {}", e),
        })?;

    let response = PaymentOrderResponse {
        order_id: req.out_trade_no,
        qr_code: result.qr_code,
        app_pay_data: None, // 需要根据实际返回数据处理
        web_pay_url: None,
    };

    Ok(ApiResponse::success(response))
}

/// 创建微信支付订单
#[utoipa::path(
    post,
    path = "/api/payment/wechat/create",
    request_body = CreateWechatOrderReq,
    responses(
        (status = 200, description = "Success", body = PaymentOrderResponse)
    )
)]
pub async fn create_wechat_order(
    State(state): State<AppState>,
    Json(req): Json<CreateWechatOrderReq>,
) -> Result<ApiResponse<PaymentOrderResponse>, AppError> {
    // 从配置或数据库获取微信支付配置
    let config = get_wechat_config(&state).await?;
    let payment = pay::wechat::Payment::new(config);

    let trade_type = match req.trade_type.as_str() {
        "JSAPI" => pay::wechat::prelude::TradeType::JSAPI,
        "NATIVE" => pay::wechat::prelude::TradeType::NATIVE,
        _ => {
            return Err(AppError::InternalError {
                message: "不支持的支付类型".to_string(),
            });
        }
    };

    let mut order_data = pay::wechat::prelude::ReqOrderBody {
        out_trade_no: req.out_trade_no.clone(),
        description: req.description,
        amount: pay::wechat::prelude::ReqAmountInfo {
            total: req.total,
            currency: Some("CNY".to_string()),
        },
        ..Default::default()
    };

    // JSAPI支付需要openid
    if trade_type == pay::wechat::prelude::TradeType::JSAPI {
        if let Some(openid) = req.openid {
            order_data.payer = Some(pay::wechat::prelude::ReqPayerInfo {
                openid: Some(openid),
                ..Default::default()
            });
        } else {
            return Err(AppError::InternalError {
                message: "JSAPI支付需要openid".to_string(),
            });
        }
    }

    let result = payment
        .create_order(trade_type, order_data)
        .await
        .map_err(|e| AppError::InternalError {
            message: format!("微信支付订单创建失败: {}", e),
        })?;

    let response = PaymentOrderResponse {
        order_id: req.out_trade_no,
        qr_code: result.code_url,
        app_pay_data: None, // 需要根据实际返回数据处理
        web_pay_url: None,
    };

    Ok(ApiResponse::success(response))
}

/// 查询订单状态
#[utoipa::path(
    get,
    path = "/api/payment/{payment_type}/query/{out_trade_no}",
    params(
        ("payment_type" = String, description = "支付类型: alipay 或 wechat"),
        ("out_trade_no" = String, description = "商户订单号")
    ),
    responses(
        (status = 200, description = "Success")
    )
)]
pub async fn query_payment_order(
    State(state): State<AppState>,
    Path((payment_type, out_trade_no)): Path<(String, String)>,
) -> Result<ApiResponse<serde_json::Value>, AppError> {
    match payment_type.as_str() {
        "alipay" => {
            let config = get_alipay_config(&state).await?;
            let payment = Payment::new(config);
            let result = payment
                .query_order(&out_trade_no)
                .await
                .map_err(|e| AppError::internal_error(format!("查询支付宝订单失败: {}", e)))?;
            Ok(ApiResponse::success(serde_json::to_value(result).unwrap()))
        }
        "wechat" => {
            let config = get_wechat_config(&state).await?;
            let payment = Payment::new(config);
            let result = payment
                .query_order(&out_trade_no)
                .await
                .map_err(|e| AppError::internal_error(format!("查询微信支付订单失败: {}", e)))?;
            Ok(ApiResponse::success(serde_json::to_value(result).unwrap()))
        }
        _ => Err(AppError::bad_request("不支持的支付类型".to_string())),
    }
}

/// 支付异步通知处理
#[utoipa::path(
    post,
    path = "/api/payment/notify",
    request_body = String,
    responses(
        (status = 200, description = "Success")
    )
)]
pub async fn payment_notify(
    State(state): State<AppState>,
    body: String,
) -> Result<&'static str, AppError> {
    // 这里需要根据实际的通知格式来判断是支付宝还是微信支付的通知
    // 通常可以通过URL路径或者通知内容来判断

    // 示例：支付宝通知处理
    let alipay_config = get_alipay_config(&state).await?;
    let alipay_payment = pay::alipay::Payment::new(alipay_config);

    match alipay_payment.notify(&body) {
        Ok(notify_data) => {
            // 处理支付成功的业务逻辑
            tracing::info!("支付宝支付成功通知: {:?}", notify_data);
            // 更新订单状态等业务逻辑
            Ok("success")
        }
        Err(e) => {
            tracing::error!("支付宝通知验证失败: {}", e);
            Err(AppError::InternalError {
                message: "通知验证失败".to_string(),
            })
        }
    }
}

// 辅助函数：获取支付宝配置
async fn get_alipay_config(state: &AppState) -> Result<pay::AlipayConfig, AppError> {
    // 这里应该从数据库或配置文件中获取支付宝配置
    // 暂时返回一个示例配置
    Ok(pay::AlipayConfig {
        app_id: "your_app_id".to_string(),
        app_private_key: "path/to/private_key.pem".to_string(),
        alipay_public_cert: "path/to/alipay_public_cert.pem".to_string(),
        notify_url: Some("https://your-domain.com/api/payment/notify".to_string()),
        is_sandbox: Some(true), // 沙盒模式
        ..Default::default()
    })
}

// 辅助函数：获取微信支付配置
async fn get_wechat_config(state: &AppState) -> Result<WechatConfig, AppError> {
    // 这里应该从数据库或配置文件中获取微信支付配置
    // 暂时返回一个示例配置
    Ok(WechatConfig {
        app_id: "your_app_id".to_string(),
        mchid: "your_mch_id".to_string(),
        mch_key: "your_mch_key".to_string(),
        apiclient_key: "path/to/apiclient_key.pem".to_string(),
        apiclient_cert: "path/to/apiclient_cert.pem".to_string(),
        notify_url: "https://your-domain.com/api/payment/notify".to_string(),
        ..Default::default()
    })
}
