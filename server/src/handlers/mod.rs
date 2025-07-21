pub mod auth;
pub mod middleware;
pub mod product_handler;
// pub mod response; // 移动到types模块
pub mod app_handler;
pub mod role_handler;
pub mod user_handler;

pub async fn handler() -> axum::response::Html<&'static str> {
    axum::response::Html("<h1>Hello, World!</h1>")
}
