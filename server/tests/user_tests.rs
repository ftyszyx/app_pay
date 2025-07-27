use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

mod helpers;

#[tokio::test]
async fn test_get_users_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/users/list?page=1&page_size=10")
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
    assert!(json["data"]["page"].is_number());
}

#[tokio::test]
async fn test_get_users_list_default_pagination() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/users/list")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_users_list_with_search() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/users/list?page=1&page_size=10&username=test")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["list"].is_array());
}

#[tokio::test]
async fn test_create_user() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let create_user_body = json!({
        "username": format!("new_user_{}", chrono::Utc::now().timestamp()),
        "password": "newuserpass123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_user_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["id"].is_number());
    assert!(json["data"]["username"].is_string());
}

#[tokio::test]
async fn test_get_user_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // 先创建一个用户
    let create_user_body = json!({
        "username": format!("get_user_{}", chrono::Utc::now().timestamp()),
        "password": "getuserpass123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_user_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let user_id = json["data"]["id"].as_i64().unwrap();

    // 然后通过 ID 获取用户
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/users/{}", user_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["id"].as_i64().unwrap(), user_id);
}

#[tokio::test]
async fn test_update_user() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // 先创建一个用户
    let create_user_body = json!({
        "username": format!("update_user_{}", chrono::Utc::now().timestamp()),
        "password": "updateuserpass123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_user_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let user_id = json["data"]["id"].as_i64().unwrap();

    // 然后更新用户
    let update_user_body = json!({
        "username": format!("updated_user_{}", chrono::Utc::now().timestamp()),
        "balance": 1000
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/users/{}", user_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_user_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["id"].as_i64().unwrap(), user_id);
}

#[tokio::test]
async fn test_delete_user() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // 先创建一个用户
    let create_user_body = json!({
        "username": format!("delete_user_{}", chrono::Utc::now().timestamp()),
        "password": "deleteuserpass123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_user_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let user_id = json["data"]["id"].as_i64().unwrap();

    // 然后删除用户
    let request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/admin/users/{}", user_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
}

#[tokio::test]
async fn test_get_nonexistent_user() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/users/99999")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let bodyjson = helpers::print_response_body_get_json(response, "get_nonexistent_user").await;
    assert_eq!(bodyjson["code"], app_server::constants::APP_NOT_FOUND);
}

#[tokio::test]
async fn test_users_without_auth() {
    let app = helpers::create_test_app().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/users/list")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
