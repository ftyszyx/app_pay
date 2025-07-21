use crate::handlers;
use crate::handlers::middleware::auth;
use axum::{ Router, middleware, routing::{delete, get, post, put}, };
use sea_orm::DatabaseConnection;
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



pub fn create_router(db_pool: DatabaseConnection) -> Router {
    let user_routes = Router::new()
        .route("/me", get(handlers::auth::get_current_user))
        .route_layer(middleware::from_fn(auth));
    let app_routes = Router::new()
        .route("/apps", get(handlers::app_handler::get_app_list))
        .route("/apps", post(handlers::app_handler::add_app))
        .route("/apps/{id}", get(handlers::app_handler::get_app_by_id))
        .route("/apps/{id}", put(handlers::app_handler::update_app))
        .route("/apps/{id}", delete(handlers::app_handler::delete_app));

    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .route("/", get(handlers::handler))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .nest("/api", user_routes)
        .nest("/api", app_routes)
        .with_state(db_pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
