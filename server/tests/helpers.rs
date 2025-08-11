use app_server::{app, constants, router};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

pub async fn create_test_app() -> Router {
    let app_state = app::init_app()
        .await
        .unwrap_or_else(|e| panic!("failed to initialize app:{}", e.to_string()));
    let app = router::create_router(app_state);
    app
}

pub async fn print_response_body_get_json(
    response: Response<Body>,
    label: &str,
) -> serde_json::Value {
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    println!("{}: status={:?}, body={}\n", label, status, json);
    json
}

#[allow(dead_code)]
pub async fn create_test_user_and_login() -> String {
    let app = create_test_app().await;
    // 注册用户
    let register_body = json!({
        "username": "testuser",
        "password": "testpass123"
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/register")
        .header("content-type", "application/json")
        .body(Body::from(register_body.to_string()))
        .unwrap();

    println!("register_body: {:?}", register_body);
    let _response = app.clone().oneshot(request).await.unwrap();

    let json = print_response_body_get_json(_response, "register_response").await;
    let code = json["code"].as_u64().unwrap();
    assert!(code == 0 || code == constants::APP_USER_ALREADY_EXISTS as u64);

    // 登录获取 token
    let login_body = json!({
        "username": "testuser",
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
    let json = print_response_body_get_json(response, "login_response").await;
    json["data"]["token"].as_str().unwrap().to_string()
}
