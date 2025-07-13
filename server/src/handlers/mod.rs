use axum::response::Html;

pub mod auth;
pub mod middleware;
pub mod product;
pub mod product_handler;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
