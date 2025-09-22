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
async fn test_get_invite_records_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/invite_records/list?page=1&page_size=10")
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
async fn test_create_invite_record() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Create a user to be the invitee
    let create_user_body = json!({
        "username": format!("invitee_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
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
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "create_user_response").await;
    let user_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create another user to be the inviter
    let create_inviter_body = json!({
        "username": format!("inviter_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_inviter_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "create_inviter_response").await;
    let inviter_user_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Now create an invite record
    let create_invite_record_body = json!({
        "user_id": user_id,
        "inviter_user_id": inviter_user_id,
        "user_info": {
            "device_type": "android",
            "invite_method": "referral_code",
            "source": "app"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/invite_records")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_invite_record_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "create_invite_record_response").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["id"].is_number());
    assert_eq!(json["data"]["user_id"], user_id);
    assert_eq!(json["data"]["inviter_user_id"], inviter_user_id);
    assert!(json["data"]["user_info"].is_object());
}

#[tokio::test]
async fn test_update_invite_record() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Create users first
    let create_user_body = json!({
        "username": format!("updateuser_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
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
    let json = print_response_body_get_json(response, "create_user_for_update").await;
    let user_id = json["data"]["id"].as_i64().unwrap() as i32;

    let create_inviter_body = json!({
        "username": format!("updateinviter_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_inviter_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_inviter_for_update").await;
    let inviter_user_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create an invite record
    let create_invite_record_body = json!({
        "user_id": user_id,
        "inviter_user_id": inviter_user_id,
        "user_info": {
            "device_type": "ios",
            "invite_method": "direct_link"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/invite_records")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_invite_record_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_invite_record_for_update").await;
    let invite_record_id = json["data"]["id"].as_i64().unwrap();

    // Update the invite record
    let update_invite_record_body = json!({
        "user_info": {
            "device_type": "android",
            "invite_method": "qr_code",
            "updated": true,
            "extra_info": "Updated via test"
        }
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/invite_records/{}", invite_record_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_invite_record_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "update_invite_record_response").await;
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["user_id"], user_id);
    assert_eq!(json["data"]["inviter_user_id"], inviter_user_id);
    assert_eq!(json["data"]["user_info"]["device_type"], "android");
    assert_eq!(json["data"]["user_info"]["invite_method"], "qr_code");
    assert!(json["data"]["user_info"]["updated"].as_bool().unwrap());
}

#[tokio::test]
async fn test_get_invite_record_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Create users first
    let create_user_body = json!({
        "username": format!("getbyiduser_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
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
    let json = print_response_body_get_json(response, "create_user_for_get_by_id").await;
    let user_id = json["data"]["id"].as_i64().unwrap() as i32;

    let create_inviter_body = json!({
        "username": format!("getbyidinviter_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_inviter_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_inviter_for_get_by_id").await;
    let inviter_user_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create an invite record
    let create_invite_record_body = json!({
        "user_id": user_id,
        "inviter_user_id": inviter_user_id,
        "user_info": {
            "device_type": "web",
            "invite_method": "email",
            "source": "website"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/invite_records")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_invite_record_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_invite_record_for_get_by_id").await;
    let invite_record_id = json["data"]["id"].as_i64().unwrap();

    // Get the invite record by ID
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/invite_records/{}", invite_record_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "get_invite_record_by_id_response").await;
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["id"], invite_record_id);
    assert_eq!(json["data"]["user_id"], user_id);
    assert_eq!(json["data"]["inviter_user_id"], inviter_user_id);
    assert_eq!(json["data"]["user_info"]["device_type"], "web");
    assert_eq!(json["data"]["user_info"]["invite_method"], "email");
    // Check that user and inviter usernames are included (from the join)
    assert!(json["data"]["user_username"].is_string());
    assert!(json["data"]["inviter_username"].is_string());
}

#[tokio::test]
async fn test_delete_invite_record() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Create users first
    let create_user_body = json!({
        "username": format!("deleteuser_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
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
    let json = print_response_body_get_json(response, "create_user_for_delete").await;
    let user_id = json["data"]["id"].as_i64().unwrap() as i32;

    let create_inviter_body = json!({
        "username": format!("deleteinviter_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_inviter_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_inviter_for_delete").await;
    let inviter_user_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create an invite record to delete
    let create_invite_record_body = json!({
        "user_id": user_id,
        "inviter_user_id": inviter_user_id,
        "user_info": {
            "device_type": "mobile",
            "invite_method": "sms"
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/invite_records")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_invite_record_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_invite_record_for_delete").await;
    let invite_record_id = json["data"]["id"].as_i64().unwrap();

    // Delete the invite record
    let request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/admin/invite_records/{}", invite_record_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "delete_invite_record_response").await;
    assert!(json["success"].as_bool().unwrap());

    // Verify the invite record is deleted by trying to get it
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/invite_records/{}", invite_record_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "delete_invite_record_response").await;
    assert_eq!(json["code"].as_u64().unwrap(), app_server::constants::APP_NOT_FOUND as u64);
}

#[tokio::test]
async fn test_invite_records_pagination() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let test_cases = vec![
        "/api/admin/invite_records/list?page=1&page_size=5",
        "/api/admin/invite_records/list?page=1&page_size=20",
        "/api/admin/invite_records/list",
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
async fn test_invite_records_search_filters() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Test various search filters
    let test_cases = vec![
        "/api/admin/invite_records/list?user_id=1",
        "/api/admin/invite_records/list?inviter_user_id=1",
        "/api/admin/invite_records/list?id=1",
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
async fn test_invite_record_comprehensive_workflow() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Create multiple users and test complete workflow
    let create_user1_body = json!({
        "username": format!("workflow_user1_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_user1_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_workflow_user1").await;
    let user1_id = json["data"]["id"].as_i64().unwrap() as i32;

    let create_user2_body = json!({
        "username": format!("workflow_user2_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/users")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_user2_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_workflow_user2").await;
    let user2_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create invite record
    let create_invite_record_body = json!({
        "user_id": user1_id,
        "inviter_user_id": user2_id,
        "user_info": {
            "campaign": "summer_2024",
            "bonus_amount": 100
        }
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/invite_records")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_invite_record_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_workflow_invite_record").await;
    let invite_record_id = json["data"]["id"].as_i64().unwrap();

    // Update it
    let update_invite_record_body = json!({
        "user_info": {
            "campaign": "summer_2024",
            "bonus_amount": 150,
            "status": "completed"
        }
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/invite_records/{}", invite_record_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_invite_record_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = print_response_body_get_json(response, "update_workflow_invite_record").await;
    assert_eq!(json["data"]["user_info"]["bonus_amount"], 150);

    // Get by ID and verify
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/invite_records/{}", invite_record_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = print_response_body_get_json(response, "get_workflow_invite_record").await;
    assert_eq!(json["data"]["user_info"]["bonus_amount"], 150);
    assert_eq!(json["data"]["user_info"]["status"], "completed");

    // List and verify it appears
    let request = Request::builder()
        .method("GET")
        .uri(&format!(
            "/api/admin/invite_records/list?user_id={}",
            user1_id
        ))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = print_response_body_get_json(response, "list_workflow_invite_records").await;
    assert!(json["data"]["list"].as_array().unwrap().len() >= 1);
}
