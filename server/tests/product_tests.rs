use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

mod helpers;

#[tokio::test]
async fn test_get_products_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/products/list?page=1&page_size=10")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["list"].is_array());
    assert!(json["data"]["total"].is_number());
}

#[tokio::test]
async fn test_products_pagination() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;
    
    let test_cases = vec![
        "/api/admin/products/list?page=1&page_size=5",
        "/api/admin/products/list",
    ];
    
    for uri in test_cases {
        let request = Request::builder()
            .method("GET")
            .uri(uri)
            .header("authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();
        
        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}