use serde::Deserialize;
use utoipa::ToSchema;
// pub enum PayMethodType{
//     AliPay{name:String,appid:String,private_key:String,public_key:String},
//     WechatPay{name:String,appid:String,mch_id:String,app_secret:String},
// }

#[derive(Deserialize, ToSchema)]
pub struct PayMethodCreatePayload {
    pub name: String,
    pub status: i16,
    pub remark: Option<String>,
    pub config: Option<serde_json::Value>,
}

#[derive(Deserialize, ToSchema)]
pub struct PayMethodUpdatePayload {
    pub name: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ListPayMethodsParams {
    pub name: Option<String>,
}
