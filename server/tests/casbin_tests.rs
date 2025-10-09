use serde_json::json;
use salvo::prelude::*;
use salvo::test::TestClient;
mod helpers;

#[tokio::test]
async fn test_add_policy_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let policy_body = json!({
        "subject": "user1",
        "object": "/api/products",
        "action": "read"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&policy_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "add_policy").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].as_bool().unwrap());
}

#[tokio::test]
async fn test_add_policy_unauthorized() {
    let app = helpers::create_test_app().await;

    let policy_body = json!({
        "subject": "user1",
        "object": "/api/products",
        "action": "read"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("content-type", "application/json", true)
        .json(&policy_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::UNAUTHORIZED));
}

#[tokio::test]
async fn test_remove_policy_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // First add a policy
    let policy_body = json!({
        "subject": "user2",
        "object": "/api/orders",
        "action": "create"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&policy_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "add_policy_for_removal").await;
    assert!(json["success"].as_bool().unwrap());

    // Then remove it
    let remove_body = json!({
        "subject": "user2",
        "object": "/api/orders",
        "action": "create"
    });

    let response = TestClient::delete(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&remove_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "remove_policy").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].as_bool().unwrap());
}

#[tokio::test]
async fn test_add_role_for_user_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let role_body = json!({
        "user": "user3",
        "role": "editor"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/roles"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&role_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "add_role").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].as_bool().unwrap());
}

#[tokio::test]
async fn test_remove_role_for_user_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // First add a role
    let role_body = json!({
        "user": "user4",
        "role": "viewer"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/roles"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&role_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "add_role_for_removal").await;
    assert!(json["success"].as_bool().unwrap());

    // Then remove it
    let remove_body = json!({
        "user": "user4",
        "role": "viewer"
    });

    let response = TestClient::delete(helpers::get_url("/api/admin/permissions/roles"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&remove_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "remove_role").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].as_bool().unwrap());
}

#[tokio::test]
async fn test_get_policies_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Add some policies first
    let policies = vec![
        json!({
            "subject": "user5",
            "object": "/api/products",
            "action": "read"
        }),
        json!({
            "subject": "user5",
            "object": "/api/products",
            "action": "create"
        }),
    ];

    for policy in policies {
        let response = TestClient::post(helpers::get_url("/api/admin/permissions/policies"))
            .add_header("content-type", "application/json", true)
            .add_header("authorization", format!("Bearer {}", token), true)
            .json(&policy)
            .send(&app)
            .await;
        let json = helpers::print_response_body_get_json(response, "add_policy_for_get").await;
        assert!(json["success"].as_bool().unwrap());
    }

    // Get all policies
    let response = TestClient::get(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "get_policies").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].is_array());

    let policies_array = json["data"].as_array().unwrap();
    assert!(policies_array.len() >= 2);

    // Check that our added policies are present
    let has_read_policy = policies_array.iter().any(|p| {
        p["subject"] == "user5" && p["object"] == "/api/products" && p["action"] == "read"
    });
    let has_create_policy = policies_array.iter().any(|p| {
        p["subject"] == "user5" && p["object"] == "/api/products" && p["action"] == "create"
    });

    assert!(has_read_policy);
    assert!(has_create_policy);
}

#[tokio::test]
async fn test_get_roles_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Add some roles first
    let roles = vec![
        json!({
            "user": "user6",
            "role": "admin"
        }),
        json!({
            "user": "user7",
            "role": "editor"
        }),
    ];

    for role in roles {
        let response = TestClient::post(helpers::get_url("/api/admin/permissions/roles"))
            .add_header("content-type", "application/json", true)
            .add_header("authorization", format!("Bearer {}", token), true)
            .json(&role)
            .send(&app)
            .await;
        let json = helpers::print_response_body_get_json(response, "add_role_for_get").await;
        assert!(json["success"].as_bool().unwrap());
    }

    // Get all roles
    let response = TestClient::get(helpers::get_url("/api/admin/permissions/roles"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "get_roles").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].is_array());

    let roles_array = json["data"].as_array().unwrap();
    assert!(roles_array.len() >= 2);

    // Check that our added roles are present
    let has_admin_role = roles_array
        .iter()
        .any(|r| r["user"] == "user6" && r["role"] == "admin");
    let has_editor_role = roles_array
        .iter()
        .any(|r| r["user"] == "user7" && r["role"] == "editor");

    assert!(has_admin_role);
    assert!(has_editor_role);
}

#[tokio::test]
async fn test_check_permission_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Add a policy first
    let policy_body = json!({
        "subject": "123",
        "object": "/api/test",
        "action": "read"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&policy_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "add_policy_for_check").await;
    assert!(json["success"].as_bool().unwrap());

    // Check permission
    let check_body = json!({
        "user_id": 123,
        "resource": "/api/test",
        "action": "read"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/check"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&check_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "check_permission").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].as_bool().unwrap()); // Should have permission
}

#[tokio::test]
async fn test_check_permission_denied() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Check permission for non-existent policy
    let check_body = json!({
        "user_id": 999,
        "resource": "/api/nonexistent",
        "action": "delete"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/check"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&check_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "check_permission_denied").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(!json["data"].as_bool().unwrap()); // Should NOT have permission
}

#[tokio::test]
async fn test_check_permission_invalid_user_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Check permission with invalid user_id format
    let check_body = json!({
        "user_id": "invalid",
        "resource": "/api/test",
        "action": "read"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/check"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&check_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::BAD_REQUEST));
}

#[tokio::test]
async fn test_reload_policies_success() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/reload"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "reload_policies").await;
    assert!(json["success"].as_bool().unwrap());
    assert_eq!(
        json["data"].as_str().unwrap(),
        "Policies reloaded successfully"
    );
}

#[tokio::test]
async fn test_role_based_permission_check() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Add a role-based policy
    let policy_body = json!({
        "subject": "manager",
        "object": "/api/reports",
        "action": "read"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&policy_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "add_role_policy").await;
    assert!(json["success"].as_bool().unwrap());

    // Assign role to user
    let role_body = json!({
        "user": "456",
        "role": "manager"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/roles"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&role_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "assign_role").await;
    assert!(json["success"].as_bool().unwrap());

    // Check if user has permission through role
    let check_body = json!({
        "user_id": 456,
        "resource": "/api/reports",
        "action": "read"
    });

    let response = TestClient::post(helpers::get_url("/api/admin/permissions/check"))
        .add_header("content-type", "application/json", true)
        .add_header("authorization", format!("Bearer {}", token), true)
        .json(&check_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));

    let json = helpers::print_response_body_get_json(response, "check_role_permission").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"].as_bool().unwrap()); // Should have permission through role
}

#[tokio::test]
async fn test_bulk_policy_operations() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;

    // Add multiple policies
    let policies = vec![
        ("user8", "/api/products", "read"),
        ("user8", "/api/products", "create"),
        ("user8", "/api/orders", "read"),
        ("user9", "/api/products", "read"),
    ];

    for (subject, object, action) in policies {
        let policy_body = json!({
            "subject": subject,
            "object": object,
            "action": action
        });

        let response = TestClient::post(helpers::get_url("/api/admin/permissions/policies"))
            .add_header("content-type", "application/json", true)
            .add_header("authorization", format!("Bearer {}", token), true)
            .json(&policy_body)
            .send(&app)
            .await;
        let json = helpers::print_response_body_get_json(response, "bulk_add_policy").await;
        assert!(json["success"].as_bool().unwrap());
    }

    // Verify all policies were added
    let response = TestClient::get(helpers::get_url("/api/admin/permissions/policies"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "get_bulk_policies").await;
    assert!(json["success"].as_bool().unwrap());

    let policies_array = json["data"].as_array().unwrap();

    // Check that user8 has multiple permissions
    let user8_policies: Vec<_> = policies_array
        .iter()
        .filter(|p| p["subject"] == "user8")
        .collect();
    assert!(user8_policies.len() >= 3);
}
