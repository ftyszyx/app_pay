use salvo::prelude::*;
use salvo::test::TestClient;
use std::time::{Duration, Instant};
use std::sync::Arc;
mod helpers;

#[tokio::test]
async fn test_concurrent_requests() {
    let service = helpers::create_test_app().await;
    let app = Arc::new(service);
    let token = helpers::create_test_user_and_login(&*app).await;
    let start = Instant::now();
    let mut handles = Vec::new();
    for i in 0..50 {
        let app = app.clone();
        let token = token.clone();
        let handle = tokio::spawn(async move {
            let response = TestClient::get(helpers::get_url("/api/admin/users/list?page=1&page_size=5"))
                .add_header("authorization", format!("Bearer {}", token), true)
                .send(&*app)
                .await;
            (i, response.status_code)
        });
        handles.push(handle);
    }
    let mut successful = 0;
    for handle in handles {
        let (_, status_opt) = handle.await.unwrap();
        if status_opt.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR).is_success() {
            successful += 1;
        }
    }
    let duration = start.elapsed();
    println!("Concurrent test results:");
    println!("  - {} successful requests out of 50", successful);
    println!("  - Total time: {:?}", duration);
    println!("  - Average time per request: {:?}", duration / 50);
    assert!(successful >= 45, "At least 90% of requests should succeed");
    assert!(duration < Duration::from_secs(10), "Should complete within 10 seconds");
}

#[tokio::test]
async fn test_response_times() {
    let service = helpers::create_test_app().await;
    let app = Arc::new(service);
    let token = helpers::create_test_user_and_login(&*app).await;
    let endpoints = vec![
        "/api/admin/users/list",
        "/api/admin/apps/list",
        "/api/admin/roles/list",
        "/api/admin/products/list",
        "/api/admin/pay_methods/list",
    ];
    for endpoint in endpoints {
        let start = Instant::now();
        let response = TestClient::get(helpers::get_url(endpoint))
            .add_header("authorization", format!("Bearer {}", token), true)
            .send(&*app)
            .await;
        let duration = start.elapsed();
        println!("  - {}: {:?} ({:?})", endpoint, duration, response.status_code);
        assert!(response.status_code.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR).is_success());
        assert!(duration < Duration::from_secs(2), "Response time should be under 2 seconds");
    }
}
