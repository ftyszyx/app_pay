use crate::handlers::{self, middleware::auth,middleware::error_handler};
use crate::types::common::AppState;
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::register,
        handlers::auth::login,
        // app
        handlers::app_handler::add,
        handlers::app_handler::get_list,
        handlers::app_handler::get_by_id,
        handlers::app_handler::update,
        handlers::app_handler::delete,
        //users
        handlers::user_handler::add,
        handlers::user_handler::get_by_id,
        handlers::user_handler::update,
        handlers::user_handler::delete,
        handlers::user_handler::get_list,
        //role
        handlers::role_handler::add,
        handlers::role_handler::get_list,
        handlers::role_handler::get_by_id,
        handlers::role_handler::update,
        handlers::role_handler::delete,
        //products
        handlers::product_handler::add,
        handlers::product_handler::get_list,
        handlers::product_handler::get_by_id,
        handlers::product_handler::update,
        handlers::product_handler::delete,
        //pay_methods
        handlers::pay_method_handler::add,
        handlers::pay_method_handler::get_list,
        handlers::pay_method_handler::get_by_id,
        handlers::pay_method_handler::update,
        handlers::pay_method_handler::delete,
    ),
    modifiers(&SecurityAddon),
    tags( (name = "app-pay", description = "App Pay API"))
)]
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}

pub fn create_router(app_state: AppState) -> Router {
    let admin_routes = Router::new()
        //users
        .route("/me", get(handlers::auth::get_current_user))
        .route("/users", post(handlers::user_handler::add))
        .route("/users/list", get(handlers::user_handler::get_list))
        .route("/users/{id}", get(handlers::user_handler::get_by_id))
        .route("/users/{id}", put(handlers::user_handler::update))
        .route("/users/{id}", delete(handlers::user_handler::delete))
        //apps
        .route("/apps", post(handlers::app_handler::add))
        .route("/apps/list", get(handlers::app_handler::get_list))
        .route("/apps/{id}", get(handlers::app_handler::get_by_id))
        .route("/apps/{id}", put(handlers::app_handler::update))
        .route("/apps/{id}", delete(handlers::app_handler::delete))
        //roles
        .route("/roles", post(handlers::role_handler::add))
        .route("/roles/list", get(handlers::role_handler::get_list))
        .route("/roles/{id}", get(handlers::role_handler::get_by_id))
        .route("/roles/{id}", put(handlers::role_handler::update))
        .route("/roles/{id}", delete(handlers::role_handler::delete))
        //products
        .route("/products", post(handlers::product_handler::add))
        .route("/products/list", get(handlers::product_handler::get_list))
        .route( "/products/{id}", get(handlers::product_handler::get_by_id))
        .route( "/products/{id}", put(handlers::product_handler::update))
        .route( "/products/{id}", delete(handlers::product_handler::delete))
        //pay_methods
        .route( "/pay_methods", post(handlers::pay_method_handler::add))
        .route( "/pay_methods/list", get(handlers::pay_method_handler::get_list))
        .route( "/pay_methods/{id}", get(handlers::pay_method_handler::get_by_id))
        .route( "/pay_methods/{id}", put(handlers::pay_method_handler::update))
        .route( "/pay_methods/{id}", delete(handlers::pay_method_handler::delete))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .route_layer(middleware::from_fn(error_handler));

    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .route("/", get(handlers::handler))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .nest("/api/admin", admin_routes)
        .with_state(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
