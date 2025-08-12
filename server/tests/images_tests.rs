use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

mod helpers;

fn unique_suffix() -> String {
    format!("{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0))
}

#[tokio::test]
async fn test_get_images_list() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/images/list?page=1&page_size=10")
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
async fn test_create_image() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("demo-img-{}", suffix),
        "object_key": format!("folder/demo-{}.png", suffix),
        "url": format!("https://cdn.example.com/demo-{}.png", suffix),
        "path": "/local/path/demo.png",
        "tags": ["cover", "banner"],
        "status": 1,
        "remark": "test image"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/images")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_body.to_string()))
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
async fn test_get_image_by_id() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // create first
    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("get-img-{}", suffix),
        "object_key": format!("imgs/get-{}.jpg", suffix),
        "url": format!("https://cdn.example.com/get-{}.jpg", suffix),
        "path": "/local/path/get.jpg",
        "status": 1
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/images")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_body.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let image_id = json["data"]["id"].as_i64().unwrap();

    // get by id
    let request = Request::builder()
        .method("GET")
        .uri(&format!("/api/admin/images/{}", image_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_update_image() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // create
    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("upd-img-{}", suffix),
        "object_key": format!("imgs/upd-{}.png", suffix),
        "url": format!("https://cdn.example.com/upd-{}.png", suffix),
        "path": "/local/path/upd.png",
        "status": 1
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/images")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_body.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let image_id = json["data"]["id"].as_i64().unwrap();

    // update
    let update_body = json!({
        "name": format!("upd-img-{}-new", suffix),
        "remark": "updated"
    });
    let request = Request::builder()
        .method("PUT")
        .uri(&format!("/api/admin/images/{}", image_id))
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(update_body.to_string()))
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_image() {
    let app = helpers::create_test_app().await;
    let token = helpers::create_test_user_and_login().await;

    // create
    let suffix = unique_suffix();
    let create_body = json!({
        "name": format!("del-img-{}", suffix),
        "object_key": format!("imgs/del-{}.png", suffix),
        "url": format!("https://cdn.example.com/del-{}.png", suffix),
        "path": "/local/path/del.png",
        "status": 1
    });
    let request = Request::builder()
        .method("POST")
        .uri("/api/admin/images")
        .header("authorization", format!("Bearer {}", token))
        .header("content-type", "application/json")
        .body(Body::from(create_body.to_string()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let image_id = json["data"]["id"].as_i64().unwrap();

    // delete
    let request = Request::builder()
        .method("DELETE")
        .uri(&format!("/api/admin/images/{}", image_id))
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_images_without_auth() {
    let app = helpers::create_test_app().await;

    let request = Request::builder()
        .method("GET")
        .uri("/api/admin/images/list")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}


