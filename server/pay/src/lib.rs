//! 支付模块 - 基于 weapay 集成支付宝和微信支付
//! 支持微信支付和支付宝支付rust sdk，微信支付基于api v3
//! 包名称：weapay 意为 wechat pay & alipay

pub mod alipay;
pub mod error;
pub mod utils;
pub mod wechat;

pub use error::WeaError;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

// reqwest 请求 user-agent
const SDK_UA: &str = "Weapay rust sdk/0.1.0";

pub type WeaResult<T> = Result<T, WeaError>;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = WeaResult<T>> + Send + 'a>>;

/// 微信支付配置
#[derive(Clone, Debug, Default)]
pub struct WechatConfig {
    // 服务商公众号或小程序appid
    pub sp_appid: Option<String>,
    // 服务商商户号
    pub sp_mchid: Option<String>,
    // 公众号或小程序或绑定到三方平台应用的appid,
    // 如果是服务商模式，此处填写服务商的appid
    pub app_id: String,
    // 商户号，如果是服务商模式，此处填写服务商的商户号
    pub mchid: String,
    // 商户支付密钥
    pub mch_key: String,
    // 商户证书内容文件路径
    pub apiclient_key: String,
    // 商户证书内容文件路径
    pub apiclient_cert: String,
    // 异步通知地址
    pub notify_url: String,
}

/// 支付宝支付配置
#[derive(Clone, Debug, Default)]
pub struct AlipayConfig {
    // 支付宝分配给开发者的应用ID
    pub app_id: String,
    // 应用私钥文件路径
    pub app_private_key: String,
    // 应用公钥文件路径
    pub app_public_cert: Option<String>,
    // 支付宝公钥文件路径
    pub alipay_public_cert: String,
    // 支付宝根证书文件路径
    pub alipay_root_cert: Option<String>,
    // 内容加密密钥
    pub mch_key: Option<String>,
    // 异步通知地址
    pub notify_url: Option<String>,
    // 沙盒模式
    pub is_sandbox: Option<bool>,
}

// 支付配置
pub struct Payment<T> {
    pub config: T,
}

impl<T> Payment<T>
where
    T: Debug + Clone + Default,
{
    pub fn new(config: T) -> Self {
        Payment { config }
    }
}
