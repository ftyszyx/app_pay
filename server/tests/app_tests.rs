use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

mod helpers;

#[tokio::test]
async fn test_get_apps_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;
    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/apps/list?page=1&page_size=10")
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
async fn test_create_app() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let create_app_body = json!({
        "name": format!("TestApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.test.app_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
        "app_update_info": "Test app for automated testing",
        "sort_order": 1,
        "status": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/apps")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_app_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["id"].is_number());
    assert!(json["data"]["name"].is_string());
}

#[tokio::test]
async fn test_get_app_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // 先创建一个应用
    let create_app_body = json!({
        "name": format!("GetApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.get.app_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
        "app_update_info": "Test app",
        "sort_order": 1,
        "status": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/apps")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_app_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let app_id = json["data"]["id"].as_i64().unwrap();

    // 然后通过 ID 获取应用
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/apps/{}", app_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["id"].as_i64().unwrap(), app_id);
}

#[tokio::test]
async fn test_update_app() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // 先创建一个应用
    let create_app_body = json!({
        "name": format!("UpdateApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.update.app_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
        "app_update_info": "Test app",
        "sort_order": 1,
        "status": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/apps")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_app_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let app_id = json["data"]["id"].as_i64().unwrap();

    // 然后更新应用
    let update_app_body = json!({
        "name": format!("UpdatedApp_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.1",
        "status": 2
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/apps/{}", app_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_app_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["id"].as_i64().unwrap(), app_id);
}

#[tokio::test]
async fn test_delete_app() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // 先创建一个应用
    let create_app_body = json!({
        "name": format!("DeleteApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.delete.app_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
        "app_update_info": "Test app",
        "sort_order": 1,
        "status": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/apps")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_app_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let app_id = json["data"]["id"].as_i64().unwrap();

    // 然后删除应用
    let request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/admin/apps/{}", app_id))
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
async fn test_apps_pagination() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // 测试不同的分页参数
    let test_cases = vec![
        "/api/admin/apps/list?page=1&page_size=5",
        "/api/admin/apps/list?page=2&page_size=10",
        "/api/admin/apps/list?page=1",       // 使用默认 page_size
        "/api/admin/apps/list?page_size=20", // 使用默认 page
        "/api/admin/apps/list",              // 使用所有默认值
    ];

    for uri in test_cases {
        let request = Request::builder()
            .method("GET")
            .uri(uri)
            .header("authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "Failed for URI: {}", uri);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(json["success"].as_bool().unwrap());
        assert!(json["data"]["list"].is_array());
        assert!(json["data"]["total"].is_number());
        assert!(json["data"]["page"].is_number());
    }
}
