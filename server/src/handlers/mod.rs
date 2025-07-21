pub mod auth;
pub mod middleware;
pub mod app_handler;
// pub  mod user_handler;
// pub  mod role_handler;
// pub  mod product_handler;



pub async fn handler() -> axum::response::Html<&'static str> {
    axum::response::Html("<h1>App Pay</h1>")
}
