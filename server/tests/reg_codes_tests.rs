use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

use crate::helpers::print_response_body_get_json;

mod helpers;

#[tokio::test]
async fn test_get_reg_codes_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/reg_codes/list?page=1&page_size=10")
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
async fn test_create_reg_code() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // First create an app to associate with the reg code
    let create_app_body = json!({
        "name": format!("TestApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.test.regcode_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
        "app_update_info": "Test app for reg code testing",
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
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "create_app_response").await;
    let app_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Now create a reg code
    let create_reg_code_body = json!({
        "code": format!("TESTCODE_{}", chrono::Utc::now().timestamp()),
        "app_id": app_id,
        "bind_device_info": {
            "device_type": "android",
            "device_id": "test_device_123"
        },
        "valid_days": 30,
        "max_devices": 5,
        "status": 0
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/reg_codes")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_reg_code_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "create_reg_code_response").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["id"].is_number());
    assert_eq!(json["data"]["app_id"], app_id);
    assert_eq!(json["data"]["valid_days"], 30);
    assert_eq!(json["data"]["max_devices"], 5);
    assert_eq!(json["data"]["status"], 0);
}

#[tokio::test]
async fn test_update_reg_code() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // First create an app
    let create_app_body = json!({
        "name": format!("TestApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.test.update_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
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
    let json = print_response_body_get_json(response, "create_app_for_update").await;
    let app_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create a reg code
    let create_reg_code_body = json!({
        "code": format!("UPDATETEST_{}", chrono::Utc::now().timestamp()),
        "app_id": app_id,
        "valid_days": 7,
        "max_devices": 3,
        "status": 0
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/reg_codes")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_reg_code_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_reg_code_for_update").await;
    let reg_code_id = json["data"]["id"].as_i64().unwrap();

    // Update the reg code
    let update_reg_code_body = json!({
        "valid_days": 60,
        "max_devices": 10,
        "status": 1,
        "bind_device_info": {
            "device_type": "ios",
            "updated": true
        }
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/reg_codes/{}", reg_code_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_reg_code_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "update_reg_code_response").await;
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["valid_days"], 60);
    assert_eq!(json["data"]["max_devices"], 10);
    assert_eq!(json["data"]["status"], 1);
}

#[tokio::test]
async fn test_get_reg_code_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // Create an app first
    let create_app_body = json!({
        "name": format!("TestApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.test.getbyid_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
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
    let json = print_response_body_get_json(response, "create_app_for_get_by_id").await;
    let app_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create a reg code
    let reg_code_text = format!("GETBYID_{}", chrono::Utc::now().timestamp());
    let create_reg_code_body = json!({
        "code": reg_code_text,
        "app_id": app_id,
        "valid_days": 15,
        "max_devices": 2,
        "status": 0
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/reg_codes")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_reg_code_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_reg_code_for_get_by_id").await;
    let reg_code_id = json["data"]["id"].as_i64().unwrap();

    // Get the reg code by ID
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/reg_codes/{}", reg_code_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "get_reg_code_by_id_response").await;
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["id"], reg_code_id);
    assert_eq!(json["data"]["code"], reg_code_text);
    assert_eq!(json["data"]["app_id"], app_id);
    assert_eq!(json["data"]["valid_days"], 15);
    assert_eq!(json["data"]["max_devices"], 2);
    assert!(json["data"]["app_name"].is_string());
}

#[tokio::test]
async fn test_delete_reg_code() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // Create an app first
    let create_app_body = json!({
        "name": format!("TestApp_{}", chrono::Utc::now().timestamp()),
        "app_id": format!("com.test.delete_{}", chrono::Utc::now().timestamp()),
        "app_vername": "1.0.0",
        "app_vercode": 1,
        "app_download_url": "https://example.com/download",
        "app_res_url": "https://example.com/resources",
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
    let json = print_response_body_get_json(response, "create_app_for_delete").await;
    let app_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create a reg code to delete
    let create_reg_code_body = json!({
        "code": format!("DELETE_{}", chrono::Utc::now().timestamp()),
        "app_id": app_id,
        "valid_days": 1,
        "max_devices": 1,
        "status": 0
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/reg_codes")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_reg_code_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_reg_code_for_delete").await;
    let reg_code_id = json["data"]["id"].as_i64().unwrap();

    // Delete the reg code
    let request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/admin/reg_codes/{}", reg_code_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "delete_reg_code_response").await;
    assert!(json["success"].as_bool().unwrap());

    // Verify the reg code is deleted by trying to get it
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/reg_codes/{}", reg_code_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "delete_reg_code_response").await;
    assert_eq!(json["code"].as_i64().unwrap(), app_server::constants::APP_NOT_FOUND as i64);
}

#[tokio::test]
async fn test_reg_codes_pagination() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let test_cases = vec![
        "/api/admin/reg_codes/list?page=1&page_size=5",
        "/api/admin/reg_codes/list?page=1&page_size=20",
        "/api/admin/reg_codes/list",
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

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(json["success"].as_bool().unwrap());
        assert!(json["data"]["list"].is_array());
        assert!(json["data"]["total"].is_number());
        assert!(json["data"]["page"].is_number());
    }
}

#[tokio::test]
async fn test_reg_codes_search_filters() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // Test various search filters
    let test_cases = vec![
        "/api/admin/reg_codes/list?status=0",
        "/api/admin/reg_codes/list?status=1",
        "/api/admin/reg_codes/list?code=TEST",
        "/api/admin/reg_codes/list?app_id=1",
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

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert!(json["success"].as_bool().unwrap());
        assert!(json["data"]["list"].is_array());
    }
}

#[tokio::test]
async fn test_reg_code_validation_errors() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // Test with invalid data
    let invalid_reg_code_body = json!({
        "code": "",  // Empty code should fail
        "app_id": -1,  // Invalid app_id
        "valid_days": -5,  // Negative valid_days
        "max_devices": 0,  // Zero max_devices might be invalid
        "status": 99  // Invalid status
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/reg_codes")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(invalid_reg_code_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    // Should return an error status (400 or 422)
    let json = print_response_body_get_json(response, "create_reg_code_response").await;
    assert_eq!(json["success"].as_bool().unwrap(), false);
}
