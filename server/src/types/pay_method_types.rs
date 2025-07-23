use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;
// pub enum PayMethodType{
//     AliPay{name:String,appid:String,private_key:String,public_key:String},
//     WechatPay{name:String,appid:String,mch_id:String,app_secret:String},
// }

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct PayMethodCreatePayload {
    pub name: String,
    pub status: i16,
    pub remark: Option<String>,
    pub config: Option<serde_json::Value>,
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct PayMethodUpdatePayload {
    pub name: Option<String>,
}

#[derive(Deserialize, ToSchema, Debug, Default)]
pub struct ListPayMethodsParams {
    pub id: Option<i32>,
    pub name: Option<String>,
}
