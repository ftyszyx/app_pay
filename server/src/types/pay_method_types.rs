use serde::Deserialize;
use validator::Validate;
use crate::utils::convert::from_str_optional;

use crate::types::common::ListParamsReq;
// pub enum PayMethodType{
//     AliPay{name:String,appid:String,private_key:String,public_key:String},
//     WechatPay{name:String,appid:String,mch_id:String,app_secret:String},
// }

#[derive(Deserialize, Debug, Validate)]
pub struct PayMethodCreatePayload {
    pub name: String,
    pub remark: Option<String>,
    pub config: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct PayMethodUpdatePayload {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ListPayMethodsParams {
    #[serde(flatten)]
    pub pagination: ListParamsReq,
    #[serde(deserialize_with = "from_str_optional",default)]
    pub id: Option<i32>,
    pub name: Option<String>,
}
