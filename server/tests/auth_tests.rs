use serde_json::json;
use salvo::prelude::*;
use salvo::test::TestClient;
mod helpers;

#[tokio::test]
async fn test_register_success() {
    let app = helpers::create_test_app().await;
    let register_body = json!({
        "username": format!("testuser_{}", chrono::Utc::now().timestamp()),
        "password": "testpass123"
    });
    let response = TestClient::post(helpers::get_url("/api/register"))
        .add_header("content-type", "application/json", true)
        .json(&register_body)
        .send(&app)
        .await;
    let bodyjson = helpers::print_response_body_get_json(response, "register").await;
    assert!(bodyjson["success"].as_bool().unwrap());
    assert!(bodyjson["data"]["token"].is_string());
}

#[tokio::test]
async fn test_register_duplicate_user() {
    let app = helpers::create_test_app().await;

    let username = format!("duplicate_user_{}", chrono::Utc::now().timestamp());
    let register_body = json!({
        "username": username,
        "password": "testpass123"
    });

    let response = TestClient::post(helpers::get_url("/api/register"))
        .add_header("content-type", "application/json", true)
        .json(&register_body)
        .send(&app)
        .await;
    let bodyjson = helpers::print_response_body_get_json(response, "register").await;
    assert!(bodyjson["success"].as_bool().unwrap());
    let response = TestClient::post(helpers::get_url("/api/register"))
        .add_header("content-type", "application/json", true)
        .json(&register_body)
        .send(&app)
        .await;
    let bodyjson = helpers::print_response_body_get_json(response, "register").await;
    let success = bodyjson["success"].as_bool().unwrap();
    assert!(!success);
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
    let _ = TestClient::post(helpers::get_url("/api/register"))
        .add_header("content-type", "application/json", true)
        .json(&register_body)
        .send(&app)
        .await;

    // 然后登录
    let login_body = json!({
        "username": username,
        "password": "testpass123"
    });
    let response = TestClient::post(helpers::get_url("/api/login"))
        .add_header("content-type", "application/json", true)
        .json(&login_body)
        .send(&app)
        .await;
    let bodyjson = helpers::print_response_body_get_json(response, "login").await;
    assert!(bodyjson["success"].as_bool().unwrap());
    assert!(bodyjson["data"]["token"].is_string());
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let app = helpers::create_test_app().await;
    let login_body = json!({
        "username": "nonexistent_user",
        "password": "wrongpassword"
    });
    let response = TestClient::post(helpers::get_url("/api/login"))
        .add_header("content-type", "application/json", true)
        .json(&login_body)
        .send(&app)
        .await;
    let bodyjson = helpers::print_response_body_get_json(response, "login").await;
    let success = bodyjson["success"].as_bool().unwrap();
    assert!(!success);
}

#[tokio::test]
async fn test_get_current_user() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let response = TestClient::get(helpers::get_url("/api/admin/me"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    let bodyjson = helpers::print_response_body_get_json(response, "me").await;
    assert!(bodyjson["success"].as_bool().unwrap());
    assert!(bodyjson["data"]["username"].is_string());
    assert!(bodyjson["data"]["role_name"].is_string());
}

#[tokio::test]
async fn test_unauthorized_access() {
    let app = helpers::create_test_app().await;
    let response = TestClient::get(helpers::get_url("/api/admin/me"))
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::UNAUTHORIZED));
}

#[tokio::test]
async fn test_invalid_token() {
    let app = helpers::create_test_app().await;
    let response = TestClient::get(helpers::get_url("/api/admin/me"))
        .add_header("authorization", "Bearer invalid_token", true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::UNAUTHORIZED));
}
