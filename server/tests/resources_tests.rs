use salvo::prelude::*;
use salvo::test::TestClient;
use serde_json::json;
mod helpers;

fn unique_suffix() -> String { format!("{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)) }

#[tokio::test]
async fn test_get_resources_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let response = TestClient::get(helpers::get_url("/api/admin/resources/list?page=1&page_size=10"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    let json = helpers::print_response_body_get_json(response, "get_resources_list").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["list"].is_array());
    assert!(json["data"]["total"].is_number());
}

#[tokio::test]
async fn test_create_resource() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("demo-res-{}", suffix),
        "object_key": format!("folder/demo-{}.png", suffix),
        "url": format!("https://cdn.example.com/demo-{}.png", suffix),
        "path": "/local/path/demo.png",
        "file_type": "image",
        "tags": ["cover", "banner"],
        "status": 1,
        "remark": "test resource"
    });
    let response = TestClient::post(helpers::get_url("/api/admin/resources"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
    let json = helpers::print_response_body_get_json(response, "create_resource").await;
    assert!(json["success"].as_bool().unwrap());
    assert!(json["data"]["id"].is_number());
    assert!(json["data"]["name"].is_string());
}

#[tokio::test]
async fn test_get_resource_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("get-res-{}", suffix),
        "object_key": format!("res/get-{}.jpg", suffix),
        "url": format!("https://cdn.example.com/get-{}.jpg", suffix),
        "path": "/local/path/get.jpg",
        "status": 1
    });
    let response = TestClient::post(helpers::get_url("/api/admin/resources"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "create_then_get_res").await;
    let resource_id = json["data"]["id"].as_i64().unwrap();
    let response = TestClient::get(helpers::get_url(&format!("/api/admin/resources/{}", resource_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
}

#[tokio::test]
async fn test_update_resource() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("upd-res-{}", suffix),
        "object_key": format!("res/upd-{}.png", suffix),
        "url": format!("https://cdn.example.com/upd-{}.png", suffix),
        "path": "/local/path/upd.png",
        "status": 1
    });
    let response = TestClient::post(helpers::get_url("/api/admin/resources"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "create_res_for_update").await;
    let resource_id = json["data"]["id"].as_i64().unwrap();
    let update_body = json!({ "name": format!("upd-res-{}-new", suffix), "remark": "updated" });
    let response = TestClient::put(helpers::get_url(&format!("/api/admin/resources/{}", resource_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&update_body)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
}

#[tokio::test]
async fn test_delete_resource() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login(&app).await;
    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("del-res-{}", suffix),
        "object_key": format!("res/del-{}.png", suffix),
        "url": format!("https://cdn.example.com/del-{}.png", suffix),
        "path": "/local/path/del.png",
        "status": 1
    });
    let response = TestClient::post(helpers::get_url("/api/admin/resources"))
        .add_header("authorization", format!("Bearer {}", token), true)
        .add_header("content-type", "application/json", true)
        .json(&create_body)
        .send(&app)
        .await;
    let json = helpers::print_response_body_get_json(response, "create_res_for_delete").await;
    let resource_id = json["data"]["id"].as_i64().unwrap();
    let response = TestClient::delete(helpers::get_url(&format!("/api/admin/resources/{}", resource_id)))
        .add_header("authorization", format!("Bearer {}", token), true)
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::OK));
}

#[tokio::test]
async fn test_resources_without_auth() {
    let app = helpers::create_test_app().await;
    let response = TestClient::get(helpers::get_url("/api/admin/resources/list"))
        .send(&app)
        .await;
    assert_eq!(response.status_code, Some(StatusCode::UNAUTHORIZED));
}
