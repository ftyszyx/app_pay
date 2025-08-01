pub mod app_handler;
pub mod auth;
pub mod crud_macro;
pub mod invite_records_handler;
pub mod middleware;
pub mod orders_handler;
pub mod pay_method_handler;
pub mod product_handler;
pub mod reg_codes_handler;
pub mod role_handler;
pub mod user_handler;

pub async fn handler() -> axum::response::Html<&'static str> {
    axum::response::Html("<h1>App Pay</h1>")
}
