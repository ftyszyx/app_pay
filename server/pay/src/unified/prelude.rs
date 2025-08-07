//! 统一支付接口的预导入模块
//! 包含了使用统一支付接口所需的所有常用类型和trait

pub use super::{
    OrderStatus, PaymentMethod, PaymentProvider, UnifiedNotifyData, UnifiedOrderRequest,
    UnifiedOrderResponse, UnifiedPayment, UnifiedPaymentConfig, UnifiedPaymentTrait,
    UnifiedQueryRequest, UnifiedQueryResponse,
};

pub use crate::{AlipayConfig, WechatConfig};
