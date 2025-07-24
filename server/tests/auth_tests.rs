 use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

mod helpers;

#[tokio::test]
async fn test_register_success() {
    let app = helpers::create_test_app().await;
    
    let register_body = json!({
        "username": format!("testuser_{}", chrono::Utc::now().timestamp()),
        "password": "testpass123"
    });
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/register")
        .header("content-type", "application/json")
        .body(Body::from(register_body.to_string()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["token"].is_string());
}

#[tokio::test]
async fn test_register_duplicate_user() {
    let app = helpers::create_test_app().await;
    
    let username = format!("duplicate_user_{}", chrono::Utc::now().timestamp());
    let register_body = json!({
        "username": username,
        "password": "testpass123"
    });
    
    // 第一次注册
    let request = Request::builder()
        .method("POST")
        .uri("/api/register")
        .header("content-type", "application/json")
        .body(Body::from(register_body.to_string()))
        .unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    // 第二次注册相同用户名
    let request = Request::builder()
        .method("POST")
        .uri("/api/register")
        .header("content-type", "application/json")
        .body(Body::from(register_body.to_string()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_login_success() {
    let app = helpers::create_test_app().await;
    
    // 先注册用户
    let username = format!("login_user_{}", chrono::Utc::now().timestamp());
    let register_body = json!({
        "username": username,
        "password": "testpass123"
    });
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/register")
        .header("content-type", "application/json")
        .body(Body::from(register_body.to_string()))
        .unwrap();
    
    let _response = app.clone().oneshot(request).await.unwrap();
    
    // 然后登录
    let login_body = json!({
        "username": username,
        "password": "testpass123"
    });
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/login")
        .header("content-type", "application/json")
        .body(Body::from(login_body.to_string()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["token"].is_string());
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let app = helpers::create_test_app().await;
    
    let login_body = json!({
        "username": "nonexistent_user",
        "password": "wrongpassword"
    });
    
    let request = Request::builder()
        .method("POST")
        .uri("/api/login")
        .header("content-type", "application/json")
        .body(Body::from(login_body.to_string()))
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_current_user() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/me")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["username"].is_string());
    assert!(json["data"]["role"].is_string());
}

#[tokio::test]
async fn test_unauthorized_access() {
    let app = helpers::create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/me")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_invalid_token() {
    let app = helpers::create_test_app().await;
    
    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/me")
        .header("authorization", "Bearer invalid_token")
        .body(Body::empty())
        .unwrap();
    
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}