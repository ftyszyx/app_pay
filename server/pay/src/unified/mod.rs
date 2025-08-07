//! 统一支付接口
//! 提供支付宝和微信支付的统一接口

use crate::error::WeaError;
use crate::{AlipayConfig, Payment, WeaResult, WechatConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

// 为统一接口定义专门的 BoxFuture 类型
pub type UnifiedBoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub mod prelude;

/// 支付提供商
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentProvider {
    /// 微信支付
    Wechat,
    /// 支付宝
    Alipay,
}

/// 支付方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentMethod {
    /// APP支付
    App,
    /// 网页支付
    Web,
    /// 扫码支付
    QrCode,
    /// 小程序支付
    MiniProgram,
    /// H5支付
    H5,
}

/// 统一支付配置
#[derive(Debug, Clone)]
pub struct UnifiedPaymentConfig {
    /// 微信支付配置
    pub wechat: Option<WechatConfig>,
    /// 支付宝配置
    pub alipay: Option<AlipayConfig>,
}

/// 统一订单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedOrderRequest {
    /// 商户订单号
    pub out_trade_no: String,
    /// 订单描述
    pub description: String,
    /// 支付金额（分）
    pub total_amount: u64,
    /// 货币类型，默认CNY
    pub currency: Option<String>,
    /// 用户标识（微信openid或支付宝buyer_id）
    pub user_id: Option<String>,
    /// 异步通知地址
    pub notify_url: Option<String>,
    /// 订单过期时间
    pub time_expire: Option<String>,
    /// 商品标记
    pub goods_tag: Option<String>,
    /// 附加数据
    pub attach: Option<String>,
    /// 扩展参数
    pub extra: Option<HashMap<String, String>>,
}

/// 统一订单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedOrderResponse {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error_msg: Option<String>,
    /// 预支付交易会话标识
    pub prepay_id: Option<String>,
    /// 支付跳转链接
    pub pay_url: Option<String>,
    /// 二维码内容
    pub qr_code: Option<String>,
    /// 调起支付的参数（JSON格式）
    pub pay_params: Option<String>,
    /// 原始响应数据
    pub raw_response: Option<String>,
}

/// 统一查询订单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedQueryRequest {
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 第三方订单号
    pub transaction_id: Option<String>,
}

/// 统一订单状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 待支付
    Pending,
    /// 支付成功
    Success,
    /// 支付失败
    Failed,
    /// 已关闭
    Closed,
    /// 已退款
    Refunded,
    /// 部分退款
    PartialRefunded,
}

/// 统一查询订单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedQueryResponse {
    /// 是否成功
    pub success: bool,
    /// 错误信息
    pub error_msg: Option<String>,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 第三方订单号
    pub transaction_id: Option<String>,
    /// 订单状态
    pub status: Option<OrderStatus>,
    /// 支付金额（分）
    pub total_amount: Option<u64>,
    /// 实际支付金额（分）
    pub paid_amount: Option<u64>,
    /// 支付时间
    pub pay_time: Option<String>,
    /// 原始响应数据
    pub raw_response: Option<String>,
}

/// 统一通知数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedNotifyData {
    /// 商户订单号
    pub out_trade_no: String,
    /// 第三方订单号
    pub transaction_id: String,
    /// 订单状态
    pub status: OrderStatus,
    /// 支付金额（分）
    pub total_amount: u64,
    /// 实际支付金额（分）
    pub paid_amount: u64,
    /// 支付时间
    pub pay_time: String,
    /// 附加数据
    pub attach: Option<String>,
    /// 原始通知数据
    pub raw_data: String,
}

/// 统一支付接口trait
pub trait UnifiedPaymentTrait {
    /// 创建订单
    fn create_order(
        &self,
        provider: PaymentProvider,
        method: PaymentMethod,
        request: UnifiedOrderRequest,
    ) -> UnifiedBoxFuture<'_, UnifiedOrderResponse>;

    /// 查询订单
    fn query_order(
        &self,
        provider: PaymentProvider,
        request: UnifiedQueryRequest,
    ) -> UnifiedBoxFuture<'_, UnifiedQueryResponse>;

    /// 关闭订单
    fn close_order<'a>(
        &'a self,
        provider: PaymentProvider,
        out_trade_no: &'a str,
    ) -> UnifiedBoxFuture<'a, WeaResult<()>>;

    /// 处理异步通知
    fn handle_notify(
        &self,
        provider: PaymentProvider,
        notify_data: &str,
        headers: Option<HashMap<String, String>>,
    ) -> WeaResult<UnifiedNotifyData>;
}

/// 统一支付处理器
pub struct UnifiedPayment {
    config: UnifiedPaymentConfig,
    wechat_payment: Option<Payment<WechatConfig>>,
    alipay_payment: Option<Payment<AlipayConfig>>,
}

impl UnifiedPayment {
    /// 创建新的统一支付处理器
    pub fn new(config: UnifiedPaymentConfig) -> Self {
        let wechat_payment = config.wechat.as_ref().map(|c| Payment::new(c.clone()));
        let alipay_payment = config.alipay.as_ref().map(|c| Payment::new(c.clone()));

        Self {
            config,
            wechat_payment,
            alipay_payment,
        }
    }

    /// 获取微信支付实例
    fn get_wechat_payment(&self) -> WeaResult<&Payment<WechatConfig>> {
        self.wechat_payment
            .as_ref()
            .ok_or_else(|| WeaError::new("", "Wechat payment not configured".to_string()))
    }

    /// 获取支付宝支付实例
    fn get_alipay_payment(&self) -> WeaResult<&Payment<AlipayConfig>> {
        self.alipay_payment
            .as_ref()
            .ok_or_else(|| WeaError::new("", "Alipay payment not configured".to_string()))
    }
}

impl Default for UnifiedPaymentConfig {
    fn default() -> Self {
        Self {
            wechat: None,
            alipay: None,
        }
    }
}

impl UnifiedPaymentTrait for UnifiedPayment {
    fn create_order(
        &self,
        provider: PaymentProvider,
        method: PaymentMethod,
        request: UnifiedOrderRequest,
    ) -> UnifiedBoxFuture<'_, UnifiedOrderResponse> {
        let fut = async move {
            match provider {
                PaymentProvider::Wechat => self.create_wechat_order(method, request).await,
                PaymentProvider::Alipay => self.create_alipay_order(method, request).await,
            }
        };
        Box::pin(fut)
    }

    fn query_order(
        &self,
        provider: PaymentProvider,
        request: UnifiedQueryRequest,
    ) -> UnifiedBoxFuture<'_, UnifiedQueryResponse> {
        let fut = async move {
            match provider {
                PaymentProvider::Wechat => self.query_wechat_order(request).await,
                PaymentProvider::Alipay => self.query_alipay_order(request).await,
            }
        };
        Box::pin(fut)
    }

    fn close_order<'a>(
        &'a self,
        provider: PaymentProvider,
        out_trade_no: &'a str,
    ) -> UnifiedBoxFuture<'a, WeaResult<()>> {
        let fut = async move {
            match provider {
                PaymentProvider::Wechat => {
                    let payment = self.get_wechat_payment()?;
                    use crate::wechat::common::BaseTrait;
                    payment.close_order(out_trade_no).await.map_err(|e| e)
                }
                PaymentProvider::Alipay => {
                    let payment = self.get_alipay_payment()?;
                    use crate::alipay::common::BaseTrait;
                    use crate::alipay::prelude::ReqCloseOrderBody;
                    let close_request = ReqCloseOrderBody {
                        out_trade_no: Some(out_trade_no.to_string()),
                        ..Default::default()
                    };
                    payment
                        .close_order(close_request)
                        .await
                        .map(|_| ())
                        .map_err(|e| e)
                }
            }
        };
        Box::pin(fut)
    }

    fn handle_notify(
        &self,
        provider: PaymentProvider,
        notify_data: &str,
        headers: Option<HashMap<String, String>>,
    ) -> WeaResult<UnifiedNotifyData> {
        match provider {
            PaymentProvider::Wechat => self.handle_wechat_notify(notify_data, headers),
            PaymentProvider::Alipay => self.handle_alipay_notify(notify_data),
        }
    }
}

impl UnifiedPayment {
    /// 创建微信订单
    async fn create_wechat_order(
        &self,
        method: PaymentMethod,
        request: UnifiedOrderRequest,
    ) -> UnifiedOrderResponse {
        use crate::wechat::common::BaseTrait;
        use crate::wechat::prelude::*;

        let payment = match self.get_wechat_payment() {
            Ok(p) => p,
            Err(e) => {
                return UnifiedOrderResponse {
                    success: false,
                    error_msg: Some(e.to_string()),
                    ..Default::default()
                };
            }
        };

        let trade_type = match method {
            PaymentMethod::App => TradeType::App,
            PaymentMethod::Web => TradeType::NATIVE,
            PaymentMethod::QrCode => TradeType::NATIVE,
            PaymentMethod::MiniProgram => TradeType::JSAPI,
            PaymentMethod::H5 => TradeType::MWEB,
        };

        let wechat_request = ReqOrderBody {
            description: request.description,
            out_trade_no: request.out_trade_no,
            time_expire: request.time_expire,
            goods_tag: request.goods_tag,
            attach: request.attach,
            amount: ReqAmountInfo {
                total: request.total_amount as i32,
                currency: request.currency,
            },
            payer: request.user_id.map(|openid| PayerInfo { openid }),
            notify_url: request.notify_url,
            ..Default::default()
        };

        match payment.create_order(trade_type, wechat_request).await {
            Ok(result) => match result {
                CreateOrderResult::JSAPI(jsapi) => UnifiedOrderResponse {
                    success: true,
                    prepay_id: Some(jsapi.package.replace("prepay_id=", "")),
                    pay_params: Some(serde_json::to_string(&jsapi).unwrap_or_default()),
                    raw_response: Some(serde_json::to_string(&jsapi).unwrap_or_default()),
                    ..Default::default()
                },
                CreateOrderResult::APP(app) => UnifiedOrderResponse {
                    success: true,
                    prepay_id: Some(app.prepay_id.clone()),
                    pay_params: Some(serde_json::to_string(&app).unwrap_or_default()),
                    raw_response: Some(serde_json::to_string(&app).unwrap_or_default()),
                    ..Default::default()
                },
                CreateOrderResult::Default(default) => {
                    let qr_code = default.code_url.clone();
                    let pay_url = default.code_url.clone();
                    let prepay_id = default.prepay_id.clone();
                    let raw_response = serde_json::to_string(&default).unwrap_or_default();
                    UnifiedOrderResponse {
                        success: true,
                        prepay_id,
                        pay_url,
                        qr_code,
                        raw_response: Some(raw_response),
                        ..Default::default()
                    }
                }
            },
            Err(e) => UnifiedOrderResponse {
                success: false,
                error_msg: Some(e.to_string()),
                ..Default::default()
            },
        }
    }

    /// 创建支付宝订单
    async fn create_alipay_order(
        &self,
        method: PaymentMethod,
        request: UnifiedOrderRequest,
    ) -> UnifiedOrderResponse {
        use crate::alipay::common::BaseTrait;
        use crate::alipay::prelude::*;

        let payment = match self.get_alipay_payment() {
            Ok(p) => p,
            Err(e) => {
                return UnifiedOrderResponse {
                    success: false,
                    error_msg: Some(e.to_string()),
                    ..Default::default()
                };
            }
        };

        let (alipay_method, product_code) = match method {
            PaymentMethod::App => (
                "alipay.trade.app.pay",
                Some("QUICK_MSECURITY_PAY".to_string()),
            ),
            PaymentMethod::Web => (
                "alipay.trade.page.pay",
                Some("FAST_INSTANT_TRADE_PAY".to_string()),
            ),
            PaymentMethod::QrCode => (
                "alipay.trade.precreate",
                Some("FACE_TO_FACE_PAYMENT".to_string()),
            ),
            PaymentMethod::MiniProgram => ("alipay.trade.create", Some("JSAPI_PAY".to_string())),
            PaymentMethod::H5 => ("alipay.trade.wap.pay", Some("QUICK_WAP_WAY".to_string())),
        };

        let alipay_request = ReqOrderBody {
            out_trade_no: request.out_trade_no,
            total_amount: format!("{:.2}", request.total_amount as f64 / 100.0), // 转换为元
            subject: request.description,
            product_code,
            buyer_id: request.user_id,
            notify_url: request.notify_url,
            time_expire: request.time_expire,
            body: request.attach,
            ..Default::default()
        };

        match payment.create_order(alipay_method, alipay_request).await {
            Ok(result) => {
                let prepay_id = result.trade_no.clone();
                let pay_url = result.page_redirection_data.clone();
                let qr_code = result.qr_code.clone();
                let raw_response = serde_json::to_string(&result).unwrap_or_default();
                UnifiedOrderResponse {
                    success: true,
                    prepay_id,
                    pay_url,
                    qr_code,
                    raw_response: Some(raw_response),
                    ..Default::default()
                }
            }
            Err(e) => UnifiedOrderResponse {
                success: false,
                error_msg: Some(e.to_string()),
                ..Default::default()
            },
        }
    }

    /// 查询微信订单
    async fn query_wechat_order(&self, request: UnifiedQueryRequest) -> UnifiedQueryResponse {
        use crate::wechat::common::BaseTrait;

        let payment = match self.get_wechat_payment() {
            Ok(p) => p,
            Err(e) => {
                return UnifiedQueryResponse {
                    success: false,
                    error_msg: Some(e.to_string()),
                    ..Default::default()
                };
            }
        };

        let result = if let Some(out_trade_no) = request.out_trade_no {
            payment.query_order(&out_trade_no).await
        } else if let Some(transaction_id) = request.transaction_id {
            payment.query_order_by_transaction_id(&transaction_id).await
        } else {
            return UnifiedQueryResponse {
                success: false,
                error_msg: Some("out_trade_no or transaction_id is required".to_string()),
                ..Default::default()
            };
        };

        match result {
            Ok(order) => {
                let status = match order.trade_state {
                    crate::wechat::prelude::TradeState::SUCCESS => Some(OrderStatus::Success),
                    crate::wechat::prelude::TradeState::NOTPAY => Some(OrderStatus::Pending),
                    crate::wechat::prelude::TradeState::CLOSED => Some(OrderStatus::Closed),
                    crate::wechat::prelude::TradeState::REFUND => Some(OrderStatus::Refunded),
                    crate::wechat::prelude::TradeState::PAYERROR => Some(OrderStatus::Failed),
                    _ => None,
                };

                let total_amount = Some(order.amount.total as u64);
                let paid_amount = Some(order.amount.payer_total as u64);
                let pay_time = Some(order.success_time.clone());
                let raw_response = serde_json::to_string(&order).unwrap_or_default();

                UnifiedQueryResponse {
                    success: true,
                    out_trade_no: Some(order.out_trade_no),
                    transaction_id: Some(order.transaction_id),
                    status,
                    total_amount,
                    paid_amount,
                    pay_time,
                    raw_response: Some(raw_response),
                    ..Default::default()
                }
            }
            Err(e) => UnifiedQueryResponse {
                success: false,
                error_msg: Some(e.to_string()),
                ..Default::default()
            },
        }
    }

    /// 查询支付宝订单
    async fn query_alipay_order(&self, request: UnifiedQueryRequest) -> UnifiedQueryResponse {
        use crate::alipay::common::BaseTrait;

        let payment = match self.get_alipay_payment() {
            Ok(p) => p,
            Err(e) => {
                return UnifiedQueryResponse {
                    success: false,
                    error_msg: Some(e.to_string()),
                    ..Default::default()
                };
            }
        };

        let result = if let Some(out_trade_no) = request.out_trade_no {
            payment.query_order(&out_trade_no).await
        } else if let Some(trade_no) = request.transaction_id {
            payment.query_order_by_trade_no(&trade_no).await
        } else {
            return UnifiedQueryResponse {
                success: false,
                error_msg: Some("out_trade_no or transaction_id is required".to_string()),
                ..Default::default()
            };
        };

        match result {
            Ok(order) => {
                let status = match order.trade_status.as_deref() {
                    Some("TRADE_SUCCESS") => Some(OrderStatus::Success),
                    Some("WAIT_BUYER_PAY") => Some(OrderStatus::Pending),
                    Some("TRADE_CLOSED") => Some(OrderStatus::Closed),
                    Some("TRADE_FINISHED") => Some(OrderStatus::Success),
                    _ => None,
                };

                let total_amount = order
                    .total_amount
                    .as_ref()
                    .and_then(|a| a.parse::<f64>().ok())
                    .map(|a| (a * 100.0) as u64);

                let paid_amount = order
                    .receipt_amount
                    .as_ref()
                    .and_then(|a| a.parse::<f64>().ok())
                    .map(|a| (a * 100.0) as u64);

                let pay_time = order.gmt_payment.clone();
                let out_trade_no = order.out_trade_no.clone();
                let transaction_id = order.trade_no.clone();

                UnifiedQueryResponse {
                    success: true,
                    out_trade_no,
                    transaction_id,
                    status,
                    total_amount,
                    paid_amount,
                    pay_time,
                    raw_response: Some(serde_json::to_string(&order).unwrap_or_default()),
                    ..Default::default()
                }
            }
            Err(e) => UnifiedQueryResponse {
                success: false,
                error_msg: Some(e.to_string()),
                ..Default::default()
            },
        }
    }

    /// 处理微信通知
    fn handle_wechat_notify(
        &self,
        _notify_data: &str,
        _headers: Option<HashMap<String, String>>,
    ) -> WeaResult<UnifiedNotifyData> {
        // 这里需要实现微信支付通知的处理逻辑
        // 由于微信支付通知处理比较复杂，需要验签等步骤，这里先返回一个基本实现
        Err(WeaError::new(
            "",
            "Wechat notify handling not implemented yet".to_string(),
        ))
    }

    /// 处理支付宝通知
    fn handle_alipay_notify(&self, notify_data: &str) -> WeaResult<UnifiedNotifyData> {
        use crate::alipay::common::BaseTrait;

        let payment = self.get_alipay_payment()?;
        let notify_result = payment.notify(notify_data)?;

        let status = match notify_result.trade_status.as_str() {
            "TRADE_SUCCESS" => OrderStatus::Success,
            "WAIT_BUYER_PAY" => OrderStatus::Pending,
            "TRADE_CLOSED" => OrderStatus::Closed,
            "TRADE_FINISHED" => OrderStatus::Success,
            _ => OrderStatus::Failed,
        };

        let total_amount = notify_result
            .total_amount
            .parse::<f64>()
            .map_err(|_| WeaError::new("", "Invalid total_amount".to_string()))?;
        let total_amount = (total_amount * 100.0) as u64;

        let paid_amount = notify_result
            .receipt_amount
            .parse::<f64>()
            .map_err(|_| WeaError::new("", "Invalid receipt_amount".to_string()))?;
        let paid_amount = (paid_amount * 100.0) as u64;

        Ok(UnifiedNotifyData {
            out_trade_no: notify_result.out_trade_no,
            transaction_id: notify_result.trade_no,
            status,
            total_amount,
            paid_amount,
            pay_time: notify_result.gmt_payment.unwrap_or_else(|| "".to_string()),
            attach: None, // 支付宝通知中没有attach字段
            raw_data: notify_data.to_string(),
        })
    }
}

impl Default for UnifiedOrderResponse {
    fn default() -> Self {
        Self {
            success: false,
            error_msg: None,
            prepay_id: None,
            pay_url: None,
            qr_code: None,
            pay_params: None,
            raw_response: None,
        }
    }
}

impl Default for UnifiedQueryResponse {
    fn default() -> Self {
        Self {
            success: false,
            error_msg: None,
            out_trade_no: None,
            transaction_id: None,
            status: None,
            total_amount: None,
            paid_amount: None,
            pay_time: None,
            raw_response: None,
        }
    }
}
