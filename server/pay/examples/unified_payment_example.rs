use pay::unified::prelude::*;
use std::collections::HashMap;

/// 统一支付接口使用示例
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 配置微信支付
    let wechat_config = WechatConfig {
        app_id: "your_wechat_app_id".to_string(),
        mchid: "your_wechat_mchid".to_string(),
        mch_key: "your_wechat_mch_key".to_string(),
        apiclient_key: "path/to/apiclient_key.pem".to_string(),
        apiclient_cert: "path/to/apiclient_cert.pem".to_string(),
        notify_url: "https://your-domain.com/api/payment/wechat/notify".to_string(),
        ..Default::default()
    };

    // 配置支付宝
    let alipay_config = AlipayConfig {
        app_id: "your_alipay_app_id".to_string(),
        app_private_key: "path/to/alipay_private_key.txt".to_string(),
        alipay_public_cert: "path/to/alipay_public_cert.crt".to_string(),
        notify_url: Some("https://your-domain.com/api/payment/alipay/notify".to_string()),
        is_sandbox: Some(true), // 沙盒环境
        ..Default::default()
    };

    // 创建统一支付配置
    let config = UnifiedPaymentConfig {
        wechat: Some(wechat_config),
        alipay: Some(alipay_config),
    };

    // 创建统一支付处理器
    let payment = UnifiedPayment::new(config);

    // 示例1：创建微信APP支付订单
    println!("=== 创建微信APP支付订单 ===");
    let wechat_request = UnifiedOrderRequest {
        out_trade_no: "ORDER_20240101_001".to_string(),
        description: "测试商品".to_string(),
        total_amount: 100, // 1元，单位为分
        currency: Some("CNY".to_string()),
        user_id: Some("user_openid_123".to_string()),
        notify_url: None, // 使用配置中的默认通知地址
        time_expire: None,
        goods_tag: Some("test".to_string()),
        attach: Some("附加数据".to_string()),
        extra: Some(HashMap::new()),
    };

    let result = payment
        .create_order(PaymentProvider::Wechat, PaymentMethod::App, wechat_request)
        .await;

    if result.success {
        println!("微信支付订单创建成功！");
        if let Some(pay_params) = result.pay_params {
            println!("调起支付参数: {}", pay_params);
        }
    } else {
        println!("微信支付订单创建失败: {:?}", result.error_msg);
    }

    // 示例2：创建支付宝扫码支付订单
    println!("\n=== 创建支付宝扫码支付订单 ===");
    let alipay_request = UnifiedOrderRequest {
        out_trade_no: "ORDER_20240101_002".to_string(),
        description: "测试商品2".to_string(),
        total_amount: 200, // 2元，单位为分
        currency: Some("CNY".to_string()),
        user_id: None, // 扫码支付不需要用户ID
        notify_url: None,
        time_expire: None,
        goods_tag: None,
        attach: None,
        extra: None,
    };

    let result = payment
        .create_order(
            PaymentProvider::Alipay,
            PaymentMethod::QrCode,
            alipay_request,
        )
        .await;

    if result.success {
        println!("支付宝扫码支付订单创建成功！");
        if let Some(qr_code) = result.qr_code {
            println!("二维码内容: {}", qr_code);
        }
    } else {
        println!("支付宝扫码支付订单创建失败: {:?}", result.error_msg);
    }

    // 示例3：查询订单状态
    println!("\n=== 查询订单状态 ===");
    let query_request = UnifiedQueryRequest {
        out_trade_no: Some("ORDER_20240101_001".to_string()),
        transaction_id: None,
    };

    let result = payment
        .query_order(PaymentProvider::Wechat, query_request)
        .await;

    if result.success {
        println!("订单查询成功！");
        println!("订单状态: {:?}", result.status);
        println!("支付金额: {:?}", result.total_amount);
    } else {
        println!("订单查询失败: {:?}", result.error_msg);
    }

    // 示例4：关闭订单
    println!("\n=== 关闭订单 ===");
    match payment
        .close_order(PaymentProvider::Wechat, "ORDER_20240101_001")
        .await
    {
        Ok(_) => println!("订单关闭成功！"),
        Err(e) => println!("订单关闭失败: {}", e),
    }

    // 示例5：处理支付通知（支付宝）
    println!("\n=== 处理支付通知 ===");
    let notify_data = "gmt_create=2024-01-01+10%3A00%3A00&out_trade_no=ORDER_20240101_002&trade_status=TRADE_SUCCESS&total_amount=2.00&trade_no=2024010122001234567890123456";

    match payment.handle_notify(PaymentProvider::Alipay, notify_data, None) {
        Ok(notify_result) => {
            println!("通知处理成功！");
            println!("订单号: {}", notify_result.out_trade_no);
            println!("支付状态: {:?}", notify_result.status);
            println!("支付金额: {} 分", notify_result.total_amount);
        }
        Err(e) => println!("通知处理失败: {}", e),
    }

    Ok(())
}
