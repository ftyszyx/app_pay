use salvo::prelude::*;
use salvo::test::TestClient;
mod helpers;

#[tokio::test]
async fn test_not_found() {
    let app = helpers::create_test_app().await;
    let response = TestClient::get(helpers::get_url("/nonexistent"))
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::NOT_FOUND));
}

#[tokio::test]
async fn test_invalid_json() {
    let app = helpers::create_test_app().await;
    
    let response = TestClient::post(helpers::get_url("/api/login"))
        .add_header("content-type", "application/json", true)
        .raw_json("invalid json")
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::BAD_REQUEST));
}

#[tokio::test]
async fn test_missing_content_type() {
    let app = helpers::create_test_app().await;
    
    let response = TestClient::post(helpers::get_url("/api/login"))
        .raw_json(r#"{"username":"test","password":"test"}"#)
        .send(&app)
        .await;
    // 应该能正常处理或返回适当错误
    assert!(response.status_code.unwrap().is_client_error() || response.status_code.unwrap().is_success());
}