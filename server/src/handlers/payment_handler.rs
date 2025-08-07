use crate::types::error::AppError;
use crate::types::{common::AppState, pay_types::*, response::ApiResponse};
use axum::{
    Json,
    extract::{Path, State},
};
use pay::unified::prelude::*;
use std::collections::HashMap;

/// 创建统一支付订单
#[utoipa::path(
    post,
    path = "/api/payment/create",
    request_body = CreatePaymentOrderReq,
    responses(
        (status = 200, description = "Success", body = PaymentOrderResponse)
    )
)]
pub async fn create_payment_order(
    State(state): State<AppState>,
    Json(req): Json<CreatePaymentOrderReq>,
) -> Result<ApiResponse<PaymentOrderResponse>, AppError> {
    // 获取统一支付配置
    let config = get_unified_payment_config(&state).await?;
    let payment = UnifiedPayment::new(config);

    // 转换支付提供商
    let provider = match req.provider.as_str() {
        "wechat" => PaymentProvider::Wechat,
        "alipay" => PaymentProvider::Alipay,
        _ => {
            return Err(AppError::BadRequest(
                "Unsupported payment provider".to_string(),
            ));
        }
    };

    // 转换支付方式
    let method = match req.payment_method.as_str() {
        "app" => PaymentMethod::App,
        "web" => PaymentMethod::Web,
        "qr" => PaymentMethod::QrCode,
        "miniprogram" => PaymentMethod::MiniProgram,
        "h5" => PaymentMethod::H5,
        _ => {
            return Err(AppError::BadRequest(
                "Unsupported payment method".to_string(),
            ));
        }
    };

    // 构建统一订单请求
    let unified_request = UnifiedOrderRequest {
        out_trade_no: req.out_trade_no,
        description: req.description,
        total_amount: req.total_amount,
        currency: req.currency,
        user_id: req.user_id,
        notify_url: req.notify_url,
        time_expire: req.time_expire,
        goods_tag: req.goods_tag,
        attach: req.attach,
        extra: req.extra,
    };

    // 创建订单
    let result = payment
        .create_order(provider, method, unified_request)
        .await;

    if result.success {
        Ok(ApiResponse::success(PaymentOrderResponse {
            success: true,
            prepay_id: result.prepay_id,
            pay_url: result.pay_url,
            qr_code: result.qr_code,
            pay_params: result.pay_params,
            error_msg: None,
        }))
    } else {
        Err(AppError::PaymentError(
            result
                .error_msg
                .unwrap_or_else(|| "Payment creation failed".to_string()),
        ))
    }
}

/// 查询支付订单
#[utoipa::path(
    get,
    path = "/api/payment/{provider}/query/{out_trade_no}",
    params(
        ("provider" = String, description = "Payment provider (wechat/alipay)"),
        ("out_trade_no" = String, description = "Merchant order number")
    ),
    responses(
        (status = 200, description = "Success", body = PaymentQueryResponse)
    )
)]
pub async fn query_payment_order(
    State(state): State<AppState>,
    Path((provider, out_trade_no)): Path<(String, String)>,
) -> Result<ApiResponse<PaymentQueryResponse>, AppError> {
    // 获取统一支付配置
    let config = get_unified_payment_config(&state).await?;
    let payment = UnifiedPayment::new(config);

    // 转换支付提供商
    let provider = match provider.as_str() {
        "wechat" => PaymentProvider::Wechat,
        "alipay" => PaymentProvider::Alipay,
        _ => {
            return Err(AppError::BadRequest(
                "Unsupported payment provider".to_string(),
            ));
        }
    };

    let query_request = UnifiedQueryRequest {
        out_trade_no: Some(out_trade_no),
        transaction_id: None,
    };

    let result = payment.query_order(provider, query_request).await;

    if result.success {
        Ok(ApiResponse::success(PaymentQueryResponse {
            success: true,
            out_trade_no: result.out_trade_no,
            transaction_id: result.transaction_id,
            status: result.status.map(|s| match s {
                OrderStatus::Pending => "pending".to_string(),
                OrderStatus::Success => "success".to_string(),
                OrderStatus::Failed => "failed".to_string(),
                OrderStatus::Closed => "closed".to_string(),
                OrderStatus::Refunded => "refunded".to_string(),
                OrderStatus::PartialRefunded => "partial_refunded".to_string(),
            }),
            total_amount: result.total_amount,
            paid_amount: result.paid_amount,
            pay_time: result.pay_time,
            error_msg: None,
        }))
    } else {
        Err(AppError::PaymentError(
            result
                .error_msg
                .unwrap_or_else(|| "Query failed".to_string()),
        ))
    }
}

/// 关闭支付订单
#[utoipa::path(
    post,
    path = "/api/payment/{provider}/close/{out_trade_no}",
    params(
        ("provider" = String, description = "Payment provider (wechat/alipay)"),
        ("out_trade_no" = String, description = "Merchant order number")
    ),
    responses(
        (status = 200, description = "Success", body = ApiResponse<String>)
    )
)]
pub async fn close_payment_order(
    State(state): State<AppState>,
    Path((provider, out_trade_no)): Path<(String, String)>,
) -> Result<ApiResponse<String>, AppError> {
    // 获取统一支付配置
    let config = get_unified_payment_config(&state).await?;
    let payment = UnifiedPayment::new(config);

    // 转换支付提供商
    let provider = match provider.as_str() {
        "wechat" => PaymentProvider::Wechat,
        "alipay" => PaymentProvider::Alipay,
        _ => {
            return Err(AppError::BadRequest(
                "Unsupported payment provider".to_string(),
            ));
        }
    };

    match payment.close_order(provider, &out_trade_no).await {
        Ok(_) => Ok(ApiResponse::success(
            "Order closed successfully".to_string(),
        )),
        Err(e) => Err(AppError::PaymentError(e.to_string())),
    }
}

/// 处理支付通知
#[utoipa::path(
    post,
    path = "/api/payment/{provider}/notify",
    params(
        ("provider" = String, description = "Payment provider (wechat/alipay)")
    ),
    responses(
        (status = 200, description = "Success", body = PaymentNotifyResponse)
    )
)]
pub async fn handle_payment_notify(
    State(state): State<AppState>,
    Path(provider): Path<String>,
    body: String,
) -> Result<ApiResponse<PaymentNotifyResponse>, AppError> {
    // 获取统一支付配置
    let config = get_unified_payment_config(&state).await?;
    let payment = UnifiedPayment::new(config);

    // 转换支付提供商
    let provider = match provider.as_str() {
        "wechat" => PaymentProvider::Wechat,
        "alipay" => PaymentProvider::Alipay,
        _ => {
            return Err(AppError::BadRequest(
                "Unsupported payment provider".to_string(),
            ));
        }
    };

    // 对于微信支付，这里需要从请求头获取验签信息
    // 这里简化处理，实际项目中需要从axum的Headers中提取
    let headers: Option<HashMap<String, String>> = None;

    match payment.handle_notify(provider, &body, headers) {
        Ok(notify_data) => {
            // 这里可以添加业务逻辑，比如更新订单状态
            // update_order_status(&notify_data).await?;

            Ok(ApiResponse::success(PaymentNotifyResponse {
                success: true,
                out_trade_no: notify_data.out_trade_no,
                transaction_id: notify_data.transaction_id,
                status: match notify_data.status {
                    OrderStatus::Pending => "pending".to_string(),
                    OrderStatus::Success => "success".to_string(),
                    OrderStatus::Failed => "failed".to_string(),
                    OrderStatus::Closed => "closed".to_string(),
                    OrderStatus::Refunded => "refunded".to_string(),
                    OrderStatus::PartialRefunded => "partial_refunded".to_string(),
                },
                total_amount: notify_data.total_amount,
                paid_amount: notify_data.paid_amount,
                pay_time: notify_data.pay_time,
                error_msg: None,
            }))
        }
        Err(e) => Err(AppError::PaymentError(e.to_string())),
    }
}

/// 获取统一支付配置
async fn get_unified_payment_config(_state: &AppState) -> Result<UnifiedPaymentConfig, AppError> {
    // 这里应该从配置文件或数据库获取配置
    // 示例配置，实际项目中需要从环境变量或配置文件读取
    let wechat_config = Some(WechatConfig {
        app_id: "your_wechat_app_id".to_string(),
        mchid: "your_wechat_mchid".to_string(),
        mch_key: "your_wechat_mch_key".to_string(),
        apiclient_key: "path/to/apiclient_key.pem".to_string(),
        apiclient_cert: "path/to/apiclient_cert.pem".to_string(),
        notify_url: "https://your-domain.com/api/payment/wechat/notify".to_string(),
        ..Default::default()
    });

    let alipay_config = Some(AlipayConfig {
        app_id: "your_alipay_app_id".to_string(),
        app_private_key: "path/to/alipay_private_key.txt".to_string(),
        alipay_public_cert: "path/to/alipay_public_cert.crt".to_string(),
        notify_url: Some("https://your-domain.com/api/payment/alipay/notify".to_string()),
        is_sandbox: Some(true), // 沙盒环境
        ..Default::default()
    });

    Ok(UnifiedPaymentConfig {
        wechat: wechat_config,
        alipay: alipay_config,
    })
}
