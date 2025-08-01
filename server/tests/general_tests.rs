use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
mod helpers;



#[tokio::test]
async fn test_not_found() {
    let app = helpers::create_test_app().await;
    let request = Request::builder()
        .method("GET")
        .uri("/nonexistent")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}


#[tokio::test]
async fn test_invalid_json() {
    let app = helpers::create_test_app().await;
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/login")
        .header("content-type", "application/json")
        .body(Body::from("invalid json"))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_missing_content_type() {
    let app = helpers::create_test_app().await;
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/login")
        .body(Body::from(r#"{"username":"test","password":"test"}"#))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    // 应该能正常处理或返回适当错误
    assert!(response.status().is_client_error() || response.status().is_success());
}