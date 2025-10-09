use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 统一创建支付订单请求
#[derive(Deserialize, Debug)]
pub struct CreatePaymentOrderReq {
    /// 支付提供商 ("wechat" 或 "alipay")
    pub provider: String,
    /// 支付方式 ("app", "web", "qr", "miniprogram", "h5")
    pub payment_method: String,
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

/// 支付订单响应
#[derive(Serialize, Debug)]
pub struct PaymentOrderResponse {
    /// 是否成功
    pub success: bool,
    /// 预支付交易会话标识
    pub prepay_id: Option<String>,
    /// 支付跳转链接
    pub pay_url: Option<String>,
    /// 二维码内容
    pub qr_code: Option<String>,
    /// 调起支付的参数（JSON格式）
    pub pay_params: Option<String>,
    /// 错误信息
    pub error_msg: Option<String>,
}

/// 支付订单查询响应
#[derive(Serialize, Debug)]
pub struct PaymentQueryResponse {
    /// 是否成功
    pub success: bool,
    /// 商户订单号
    pub out_trade_no: Option<String>,
    /// 第三方订单号
    pub transaction_id: Option<String>,
    /// 订单状态
    pub status: Option<String>,
    /// 支付金额（分）
    pub total_amount: Option<u64>,
    /// 实际支付金额（分）
    pub paid_amount: Option<u64>,
    /// 支付时间
    pub pay_time: Option<String>,
    /// 错误信息
    pub error_msg: Option<String>,
}

/// 支付通知响应
#[derive(Serialize, Debug)]
pub struct PaymentNotifyResponse {
    /// 是否成功
    pub success: bool,
    /// 商户订单号
    pub out_trade_no: String,
    /// 第三方订单号
    pub transaction_id: String,
    /// 订单状态
    pub status: String,
    /// 支付金额（分）
    pub total_amount: u64,
    /// 实际支付金额（分）
    pub paid_amount: u64,
    /// 支付时间
    pub pay_time: String,
    /// 错误信息
    pub error_msg: Option<String>,
}

// 保留旧的类型定义以兼容现有代码
#[derive(Deserialize, Debug)]
pub struct CreateAlipayOrderReq {
    pub out_trade_no: String,
    pub total_amount: String,
    pub subject: String,
    pub payment_method: String, // "app", "web", "qr"
}

#[derive(Deserialize, Debug)]
pub struct CreateWechatOrderReq {
    pub out_trade_no: String,
    pub total: i32, // 分为单位
    pub description: String,
    pub trade_type: String,     // "JSAPI", "APP", "NATIVE"
    pub openid: Option<String>, // JSAPI时必填
}

#[derive(Deserialize,  Debug)]
pub struct PaymentNotifyReq {
    pub payment_type: String, // "alipay" or "wechat"
    pub notify_data: String,
}
