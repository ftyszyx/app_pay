use axum::{body::Body, http::Request};
use std::time::{Duration, Instant};
use tower::ServiceExt;

mod helpers;

#[tokio::test]
async fn test_concurrent_requests() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let start = Instant::now();
    let mut handles = Vec::new();
    // 创建50个并发请求
    for i in 0..50 {
        let app = app.clone();
        let token = token.clone();

        let handle = tokio::spawn(async move {
            let request = Request::builder()
                .method("GET")
                .uri("/api/admin/users/list?page=1&page_size=5")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap();

            let response = app.oneshot(request).await.unwrap();
            (i, response.status())
        });

        handles.push(handle);
    }
    // 等待所有请求完成
    let mut successful = 0;
    for handle in handles {
        let (_, status) = handle.await.unwrap();
        if status.is_success() {
            successful += 1;
        }
    }

    let duration = start.elapsed();
    println!("Concurrent test results:");
    println!("  - {} successful requests out of 50", successful);
    println!("  - Total time: {:?}", duration);
    println!("  - Average time per request: {:?}", duration / 50);
    assert!(successful >= 45, "At least 90% of requests should succeed");
    assert!(
        duration < Duration::from_secs(10),
        "Should complete within 10 seconds"
    );
}

#[tokio::test]
async fn test_response_times() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let endpoints = vec![
        "/api/admin/users/list",
        "/api/admin/apps/list",
        "/api/admin/roles/list",
        "/api/admin/products/list",
        "/api/admin/pay_methods/list",
    ];

    for endpoint in endpoints {
        let start = Instant::now();

        let request = Request::builder()
            .method("GET")
            .uri(endpoint)
            .header("authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        let duration = start.elapsed();

        println!("  - {}: {:?} ({})", endpoint, duration, response.status());

        assert!(response.status().is_success());
        assert!(
            duration < Duration::from_secs(2),
            "Response time should be under 2 seconds"
        );
    }
}
