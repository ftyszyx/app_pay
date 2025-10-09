use salvo::prelude::*;
use salvo::test::TestClient;
use serde_json::json;
use crate::helpers::print_response_body_get_json;
mod helpers;

#[tokio::test]
async fn test_get_orders_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let response = TestClient::get(helpers::get_url("/api/admin/orders/list?page=1&page_size=10"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    let json = print_response_body_get_json(response, "get_orders_list").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["list"].is_array());
    assert!(json["data"]["total"].is_number());
    assert!(json["data"]["page"].is_number());
}

#[tokio::test]
async fn test_get_order_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let pay_method_body = json!({
        "name": format!("GetByIdTestPayMethod_{}", chrono::Utc::now().timestamp()),
        "description": "Test payment method for get by id",
        "is_active": true
    });
    let response = TestClient::post(helpers::get_url("/api/admin/pay_methods"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&pay_method_body)
        .send(&app)
        .await;
    let json = print_response_body_get_json(response, "create_pay_method_for_get_by_id").await;
    let pay_method_id = json["data"]["id"].as_i64().unwrap() as i32;
    let create_user_body = json!({
        "username": format!("getbyidorderuser_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });
    let response = TestClient::post(helpers::get_url("/api/admin/users"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_user_body)
        .send(&app)
        .await;
    let json = print_response_body_get_json(response, "create_user_for_get_by_id").await;
    let user_id = json["data"]["id"].as_i64().unwrap() as i32;
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
    let response = TestClient::post(helpers::get_url("/api/admin/orders"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_order_body)
        .send(&app)
        .await;
    let json = print_response_body_get_json(response, "create_order_for_get_by_id").await;
    let order_id = json["data"]["id"].as_i64().unwrap();
    let response = TestClient::get(helpers::get_url(&format!("/api/admin/orders/{}", order_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
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
        helpers::get_url("/api/admin/orders/list?page=1&page_size=5"),
        helpers::get_url("/api/admin/orders/list?page=1&page_size=20"),
        helpers::get_url("/api/admin/orders/list"),
    ];
    for url in test_cases {
        let response = TestClient::get(url)
            .add_header("authorization", format!("Bearer {}", token), true)
            .send(&app)
            .await;
        assert_eq!(response.status_code, Some(StatusCode::OK));
        let json = print_response_body_get_json(response, "orders_pagination").await;
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
    let test_cases = vec![
        helpers::get_url("/api/admin/orders/list?status=1"),
        helpers::get_url("/api/admin/orders/list?pay_method_id=1"),
        helpers::get_url("/api/admin/orders/list?created_by=1"),
        helpers::get_url("/api/admin/orders/list?order_id=TEST"),
    ];
    for url in test_cases {
        let response = TestClient::get(url)
            .add_header("authorization", format!("Bearer {}", token), true)
            .send(&app)
            .await;
        assert_eq!(response.status_code, Some(StatusCode::OK));
        let json = print_response_body_get_json(response, "orders_filters").await;
        assert!(json["success"].as_bool().unwrap());
        assert!(json["data"]["list"].is_array());
    }
}

#[tokio::test]
async fn test_order_comprehensive_workflow() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let pay_method_body = json!({
        "name": format!("WorkflowTestPayMethod_{}", chrono::Utc::now().timestamp()),
        "description": "Test payment method for workflow",
        "is_active": true
    });
    let response = TestClient::post(helpers::get_url("/api/admin/pay_methods"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&pay_method_body)
        .send(&app)
        .await;
    let json = print_response_body_get_json(response, "create_pay_method_for_workflow").await;
    let pay_method_id = json["data"]["id"].as_i64().unwrap() as i32;
    let create_user_body = json!({
        "username": format!("workfloworderuser_{}", chrono::Utc::now().timestamp()),
        "password": "password123",
        "role_id": 1
    });
    let response = TestClient::post(helpers::get_url("/api/admin/users"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_user_body)
        .send(&app)
        .await;
    let json = print_response_body_get_json(response, "create_user_for_workflow").await;
    let user_id = json["data"]["id"].as_i64().unwrap() as i32;
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
    let response = TestClient::post(helpers::get_url("/api/admin/orders"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_order_body)
        .send(&app)
        .await;
    let json = print_response_body_get_json(response, "create_workflow_order").await;
    let order_id = json["data"]["id"].as_i64().unwrap();
    let update_to_processing = json!({
        "status": 2,
        "remark": "Order is being processed",
        "updated_by": user_id
    });
    let response = TestClient::put(helpers::get_url(&format!("/api/admin/orders/{}", order_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&update_to_processing)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    let json = print_response_body_get_json(response, "update_workflow_order_to_processing").await;
    assert_eq!(json["data"]["status"], 2);
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
    let response = TestClient::put(helpers::get_url(&format!("/api/admin/orders/{}", order_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&update_to_completed)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    let json = print_response_body_get_json(response, "update_workflow_order_to_completed").await;
    assert_eq!(json["data"]["status"], 3);
    let response = TestClient::get(helpers::get_url(&format!("/api/admin/orders/{}", order_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    let json = print_response_body_get_json(response, "get_workflow_order_final").await;
    assert_eq!(json["data"]["status"], 3);
    assert_eq!(json["data"]["user_info"]["status"], "completed");
    let response = TestClient::get(helpers::get_url(&format!("/api/admin/orders/list?status=3&pay_method_id={}", pay_method_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    let json = print_response_body_get_json(response, "list_workflow_orders").await;
    assert!(json["data"]["list"].as_array().unwrap().len() >= 1);
}
