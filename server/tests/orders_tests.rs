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
async fn test_get_orders_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/orders/list?page=1&page_size=10")
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
async fn test_get_order_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Create prerequisites
    let pay_method_body = json!({
        "name": format!("GetByIdTestPayMethod_{}", chrono::Utc::now().timestamp()),
        "description": "Test payment method for get by id",
        "is_active": true
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/pay_methods")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(pay_method_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_pay_method_for_get_by_id").await;
    let pay_method_id = json["data"]["id"].as_i64().unwrap() as i32;

    let create_user_body = json!({
        "username": format!("getbyidorderuser_{}", chrono::Utc::now().timestamp()),
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

    // Create an order
    let order_id_str = format!("GETBYID_ORDER_{}", chrono::Utc::now().timestamp());
    let create_order_body = json!({
        "order_id": order_id_str,
        "user_info": {
            "customer_name": "Bob Wilson",
            "email": "bob@example.com",
            "phone": "123-456-7890"
        },
        "status": 3,
        "pay_method_id": pay_method_id,
        "original_price": 20000,
        "final_price": 18000,
        "remark": "Order for get by id test",
        "created_by": user_id,
        "updated_by": user_id
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/orders")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_order_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_order_for_get_by_id").await;
    let order_id = json["data"]["id"].as_i64().unwrap();

    // Get the order by ID
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/orders/{}", order_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = print_response_body_get_json(response, "get_order_by_id_response").await;
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(json["data"]["id"], order_id);
    assert_eq!(json["data"]["order_id"], order_id_str);
    assert_eq!(json["data"]["status"], 3);
    assert_eq!(json["data"]["original_price"], 20000);
    assert_eq!(json["data"]["final_price"], 18000);
    assert_eq!(json["data"]["user_info"]["customer_name"], "Bob Wilson");
}


#[tokio::test]
async fn test_orders_pagination() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let test_cases = vec![
        "/api/admin/orders/list?page=1&page_size=5",
        "/api/admin/orders/list?page=1&page_size=20",
        "/api/admin/orders/list",
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
async fn test_orders_search_filters() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Test various search filters
    let test_cases = vec![
        "/api/admin/orders/list?status=1",
        "/api/admin/orders/list?pay_method_id=1",
        "/api/admin/orders/list?created_by=1",
        "/api/admin/orders/list?order_id=TEST",
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
async fn test_order_comprehensive_workflow() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Create prerequisites
    let pay_method_body = json!({
        "name": format!("WorkflowTestPayMethod_{}", chrono::Utc::now().timestamp()),
        "description": "Test payment method for workflow",
        "is_active": true
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/pay_methods")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(pay_method_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_pay_method_for_workflow").await;
    let pay_method_id = json["data"]["id"].as_i64().unwrap() as i32;

    let create_user_body = json!({
        "username": format!("workfloworderuser_{}", chrono::Utc::now().timestamp()),
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
    let json = print_response_body_get_json(response, "create_user_for_workflow").await;
    let user_id = json["data"]["id"].as_i64().unwrap() as i32;

    // Create order
    let order_id_str = format!("WORKFLOW_ORDER_{}", chrono::Utc::now().timestamp());
    let create_order_body = json!({
        "order_id": order_id_str,
        "user_info": {
            "customer_name": "Workflow Test User",
            "email": "workflow@example.com",
            "campaign": "summer_2024"
        },
        "status": 1,
        "pay_method_id": pay_method_id,
        "original_price": 50000,
        "final_price": 45000,
        "remark": "Workflow test order",
        "created_by": user_id,
        "updated_by": user_id
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/orders")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_order_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let json = print_response_body_get_json(response, "create_workflow_order").await;
    let order_id = json["data"]["id"].as_i64().unwrap();

    // Update it through different status transitions
    let update_to_processing = json!({
        "status": 2,
        "remark": "Order is being processed",
        "updated_by": user_id
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/orders/{}", order_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_to_processing.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = print_response_body_get_json(response, "update_workflow_order_to_processing").await;
    assert_eq!(json["data"]["status"], 2);

    // Update to completed
    let update_to_completed = json!({
        "status": 3,
        "remark": "Order completed successfully",
        "user_info": {
            "customer_name": "Workflow Test User",
            "email": "workflow@example.com",
            "campaign": "summer_2024",
            "completion_date": chrono::Utc::now().to_rfc3339(),
            "status": "completed"
        },
        "updated_by": user_id
    });

    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/orders/{}", order_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_to_completed.to_string()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = print_response_body_get_json(response, "update_workflow_order_to_completed").await;
    assert_eq!(json["data"]["status"], 3);

    // Get by ID and verify final state
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/orders/{}", order_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = print_response_body_get_json(response, "get_workflow_order_final").await;
    assert_eq!(json["data"]["status"], 3);
    assert_eq!(json["data"]["user_info"]["status"], "completed");

    // List and verify it appears
    let request = Request::builder()
        .method("GET")
        .uri(&format!(
            "/api/admin/orders/list?status=3&pay_method_id={}",
            pay_method_id
        ))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = print_response_body_get_json(response, "list_workflow_orders").await;
    assert!(json["data"]["list"].as_array().unwrap().len() >= 1);
}
