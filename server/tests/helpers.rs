use app_server::{create_router, types::{config::Config, common::AppState}, database, utils::redis_cache::RedisCache};
use axum::Router;
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;

pub async fn create_test_app() -> Router {
    // 加载配置
    let config = Config::from_env().expect("Failed to load test config");
    
    // 初始化数据库
    let db_pool = database::init_db(&config.database)
        .await
        .expect("Failed to connect to test database");
    
    // 运行迁移
    Migrator::up(&db_pool, None)
        .await
        .expect("Failed to run migrations");
    
    // 初始化 Redis
    let redis = RedisCache::new(&config.redis.url)
        .expect("Failed to connect to Redis");
    
    // 创建应用状态
    let app_state = AppState {
        db: db_pool,
        redis: Arc::new(redis),
        config: Arc::new(config),
    };
    
    // 创建路由
    create_router(app_state)
}

#[allow(dead_code)]
pub async fn create_test_user_and_login() -> String {
    use serde_json::json;
    use tower::ServiceExt;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    
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
    
    let _response = app.clone().oneshot(request).await.unwrap();
    
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
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    
    json["data"]["token"].as_str().unwrap().to_string()
} 