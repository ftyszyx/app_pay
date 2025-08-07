# 统一支付接口文档

## 概述

统一支付接口提供了支付宝和微信支付的统一抽象层，让开发者可以使用相同的API接口来处理不同的支付平台，简化了支付集成的复杂性。

## 主要特性

- **统一接口**: 支付宝和微信支付使用相同的API
- **统一数据类型**: 标准化的请求和响应数据结构
- **支付方式支持**: APP支付、网页支付、扫码支付、小程序支付、H5支付
- **异步通知处理**: 统一的支付通知处理机制
- **错误处理**: 统一的错误处理和响应格式

## 快速开始

### 1. 添加依赖

```toml
[dependencies]
pay = { path = "path/to/pay" }
tokio = { version = "1.0", features = ["full"] }
```

### 2. 基本使用

```rust
use pay::unified::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置支付
    let config = UnifiedPaymentConfig {
        wechat: Some(WechatConfig {
            app_id: "your_wechat_app_id".to_string(),
            mchid: "your_wechat_mchid".to_string(),
            mch_key: "your_wechat_mch_key".to_string(),
            apiclient_key: "path/to/apiclient_key.pem".to_string(),
            apiclient_cert: "path/to/apiclient_cert.pem".to_string(),
            notify_url: "https://your-domain.com/notify".to_string(),
            ..Default::default()
        }),
        alipay: Some(AlipayConfig {
            app_id: "your_alipay_app_id".to_string(),
            app_private_key: "path/to/private_key.txt".to_string(),
            alipay_public_cert: "path/to/public_cert.crt".to_string(),
            notify_url: Some("https://your-domain.com/notify".to_string()),
            ..Default::default()
        }),
    };

    // 创建支付处理器
    let payment = UnifiedPayment::new(config);

    // 创建订单
    let request = UnifiedOrderRequest {
        out_trade_no: "ORDER_001".to_string(),
        description: "测试商品".to_string(),
        total_amount: 100, // 1元，单位为分
        currency: Some("CNY".to_string()),
        user_id: Some("user_123".to_string()),
        ..Default::default()
    };

    let result = payment.create_order(
        PaymentProvider::Wechat,
        PaymentMethod::App,
        request
    ).await;

    if result.success {
        println!("订单创建成功: {:?}", result.prepay_id);
    }

    Ok(())
}
```

## API 参考

### 支付提供商 (PaymentProvider)

```rust
pub enum PaymentProvider {
    Wechat,  // 微信支付
    Alipay,  // 支付宝
}
```

### 支付方式 (PaymentMethod)

```rust
pub enum PaymentMethod {
    App,         // APP支付
    Web,         // 网页支付
    QrCode,      // 扫码支付
    MiniProgram, // 小程序支付
    H5,          // H5支付
}
```

### 订单状态 (OrderStatus)

```rust
pub enum OrderStatus {
    Pending,         // 待支付
    Success,         // 支付成功
    Failed,          // 支付失败
    Closed,          // 已关闭
    Refunded,        // 已退款
    PartialRefunded, // 部分退款
}
```

### 主要接口

#### 1. 创建订单

```rust
fn create_order(
    &self,
    provider: PaymentProvider,
    method: PaymentMethod,
    request: UnifiedOrderRequest,
) -> UnifiedBoxFuture<'_, UnifiedOrderResponse>;
```

**请求参数 (UnifiedOrderRequest):**
- `out_trade_no`: 商户订单号
- `description`: 订单描述
- `total_amount`: 支付金额（分）
- `currency`: 货币类型（可选，默认CNY）
- `user_id`: 用户标识（微信openid或支付宝buyer_id）
- `notify_url`: 异步通知地址（可选）
- `time_expire`: 订单过期时间（可选）
- `goods_tag`: 商品标记（可选）
- `attach`: 附加数据（可选）
- `extra`: 扩展参数（可选）

**响应参数 (UnifiedOrderResponse):**
- `success`: 是否成功
- `error_msg`: 错误信息
- `prepay_id`: 预支付交易会话标识
- `pay_url`: 支付跳转链接
- `qr_code`: 二维码内容
- `pay_params`: 调起支付的参数（JSON格式）
- `raw_response`: 原始响应数据

#### 2. 查询订单

```rust
fn query_order(
    &self,
    provider: PaymentProvider,
    request: UnifiedQueryRequest,
) -> UnifiedBoxFuture<'_, UnifiedQueryResponse>;
```

**请求参数 (UnifiedQueryRequest):**
- `out_trade_no`: 商户订单号（可选）
- `transaction_id`: 第三方订单号（可选）

**响应参数 (UnifiedQueryResponse):**
- `success`: 是否成功
- `error_msg`: 错误信息
- `out_trade_no`: 商户订单号
- `transaction_id`: 第三方订单号
- `status`: 订单状态
- `total_amount`: 支付金额（分）
- `paid_amount`: 实际支付金额（分）
- `pay_time`: 支付时间
- `raw_response`: 原始响应数据

#### 3. 关闭订单

```rust
fn close_order<'a>(
    &'a self,
    provider: PaymentProvider,
    out_trade_no: &'a str,
) -> UnifiedBoxFuture<'a, WeaResult<()>>;
```

#### 4. 处理异步通知

```rust
fn handle_notify(
    &self,
    provider: PaymentProvider,
    notify_data: &str,
    headers: Option<HashMap<String, String>>,
) -> WeaResult<UnifiedNotifyData>;
```

**响应参数 (UnifiedNotifyData):**
- `out_trade_no`: 商户订单号
- `transaction_id`: 第三方订单号
- `status`: 订单状态
- `total_amount`: 支付金额（分）
- `paid_amount`: 实际支付金额（分）
- `pay_time`: 支付时间
- `attach`: 附加数据
- `raw_data`: 原始通知数据

## 支付方式对照表

| 统一接口 | 微信支付 | 支付宝 |
|---------|---------|--------|
| App | APP支付 | APP支付 |
| Web | NATIVE | 网页支付 |
| QrCode | NATIVE | 当面付 |
| MiniProgram | JSAPI | 小程序支付 |
| H5 | MWEB | 手机网站支付 |

## 错误处理

所有接口都会返回统一的错误格式，包含 `success` 字段和 `error_msg` 字段。

```rust
if !result.success {
    eprintln!("支付失败: {}", result.error_msg.unwrap_or_default());
}
```

## 注意事项

1. **配置安全**: 私钥文件和敏感配置信息应妥善保管
2. **异步通知**: 微信支付的异步通知处理较为复杂，当前版本暂未完全实现
3. **金额单位**: 统一使用分作为金额单位
4. **环境配置**: 支付宝支持沙盒环境配置

## 示例项目

完整的使用示例请参考 `examples/unified_payment_example.rs` 文件。

## 更新日志

### v0.1.0
- 初始版本
- 支持微信支付和支付宝的统一接口
- 支持创建订单、查询订单、关闭订单
- 支持支付宝异步通知处理
- 支持多种支付方式
