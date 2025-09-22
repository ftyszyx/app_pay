use app_server::{app, constants, router};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use http_body_util::BodyExt;
use serde_json::json;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::env;
use std::process::Command;
use tower::ServiceExt;

async fn ensure_test_database_exists() {
    // 读取测试环境的 DATABASE_URL，并从中提取数据库名
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env.test");

    let db_name = extract_database_name(&database_url)
        .expect("failed to parse database name from DATABASE_URL");

    // 连接到管理库 postgres
    let mut admin_options: PgConnectOptions = database_url
        .parse()
        .expect("invalid DATABASE_URL for postgres connection");
    admin_options = admin_options.database("postgres");

    let admin_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_with(admin_options)
        .await
        .expect("failed to connect to postgres database for admin operations");

    // 检查数据库是否存在，不存在则创建
    let exists: Option<i32> = sqlx::query_scalar("SELECT 1 FROM pg_database WHERE datname = $1")
        .bind(&db_name)
        .fetch_optional(&admin_pool)
        .await
        .expect("failed to query pg_database");

    if exists.is_none() {
        println!("creating test database: {}", db_name);
        let create_sql = format!(
            "CREATE DATABASE \"{}\" WITH TEMPLATE template0 ENCODING 'UTF8'",
            db_name
        );
        sqlx::query(&create_sql)
            .execute(&admin_pool)
            .await
            .expect("failed to create test database");
        println!("created test database: {}", db_name);
    }
    // NOTE: 不在这里执行 init.sql；应用初始化会运行 SeaORM 迁移
}

fn run_init_sql_with_psql() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let output = Command::new("psql")
        .env("PGCLIENTENCODING", "UTF8")
        .arg(&database_url)
        .args(["-v", "ON_ERROR_STOP=1", "-q"]) // 安静模式，失败即停止
        .args(["-c", "SET client_min_messages = warning;"]) // 隐藏 NOTICE
        .args(["-f", "deploy/postgres/init/init.sql"])
        .output()
        .expect("failed to spawn psql");
    if !output.status.success() {
        panic!(
            "psql init.sql failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn extract_database_name(url: &str) -> Option<String> {
    // 形如 postgres://user:pass@host:port/dbname?params
    let without_params = url.split('?').next().unwrap_or(url);
    let pos = without_params.rfind('/')?;
    let name = &without_params[pos + 1..];
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

pub async fn create_test_app() -> Router {
    dotenvy::from_filename(".env.test").unwrap();
    // 确保测试数据库存在（若不存在则创建），再初始化应用
    ensure_test_database_exists().await;
    run_init_sql_with_psql();
    let app_state = app::init_app()
        .await
        .unwrap_or_else(|e| panic!("failed to initialize app:{}", e.to_string()));
    let app = router::create_router(app_state);
    app
}

pub async fn print_response_body_get_json(
    response: Response<Body>,
    label: &str,
) -> serde_json::Value {
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    println!("{}: status={:?}, body={}\n", label, status, json);
    json
}

#[allow(dead_code)]
pub async fn create_test_user_and_login() -> String {
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

    println!("register_body: {:?}", register_body);
    let _response = app.clone().oneshot(request).await.unwrap();

    let json = print_response_body_get_json(_response, "register_response").await;
    let code = json["code"].as_u64().unwrap();
    assert!(code == 0 || code == constants::APP_USER_ALREADY_EXISTS as u64);

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
    let json = print_response_body_get_json(response, "login_response").await;
    json["data"]["token"].as_str().unwrap().to_string()
}
