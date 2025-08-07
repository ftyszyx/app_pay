use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema, Debug)]
pub struct CreateAlipayOrderReq {
    pub out_trade_no: String,
    pub total_amount: String,
    pub subject: String,
    pub payment_method: String, // "app", "web", "qr"
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct CreateWechatOrderReq {
    pub out_trade_no: String,
    pub total: i32, // 分为单位
    pub description: String,
    pub trade_type: String,     // "JSAPI", "APP", "NATIVE"
    pub openid: Option<String>, // JSAPI时必填
}

#[derive(Serialize, ToSchema, Debug)]
pub struct PaymentOrderResponse {
    pub order_id: String,
    pub qr_code: Option<String>,
    pub app_pay_data: Option<String>,
    pub web_pay_url: Option<String>,
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct PaymentNotifyReq {
    pub payment_type: String, // "alipay" or "wechat"
    pub notify_data: String,
}
