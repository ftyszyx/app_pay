use crate::handlers;
use crate::handlers::middleware::auth;
use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};
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
        handlers::user_handler::create_user,
        handlers::user_handler::get_users_list,
        handlers::user_handler::get_user_by_id,
        handlers::user_handler::update_user,
        handlers::user_handler::delete_user,
        handlers::role_handler::create_role,
        handlers::role_handler::get_roles_list,
        handlers::role_handler::get_role_by_id,
        handlers::role_handler::update_role,
        handlers::role_handler::delete_role,
    ),
    modifiers(&SecurityAddon),
    tags( (name = "app-pay", description = "App Pay API"))
)]
struct ApiDoc;
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
        .route("/admin/users", post(handlers::user_handler::create_user))
        .route("/admin/users", get(handlers::user_handler::get_users_list))
        .route(
            "/admin/users/{id}",
            get(handlers::user_handler::get_user_by_id),
        )
        .route(
            "/admin/users/{id}",
            put(handlers::user_handler::update_user),
        )
        .route(
            "/admin/users/{id}",
            delete(handlers::user_handler::delete_user),
        )
        .route_layer(middleware::from_fn(auth));

    let role_routes = Router::new()
        .route("/admin/roles", post(handlers::role_handler::create_role))
        .route("/admin/roles", get(handlers::role_handler::get_roles_list))
        .route(
            "/admin/roles/{id}",
            get(handlers::role_handler::get_role_by_id),
        )
        .route(
            "/admin/roles/{id}",
            put(handlers::role_handler::update_role),
        )
        .route(
            "/admin/roles/{id}",
            delete(handlers::role_handler::delete_role),
        )
        .route_layer(middleware::from_fn(auth));

    let products_routes = Router::new().route(
        "/products",
        get(handlers::product_handler::get_all_products),
    );

    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .route("/", get(handlers::handler))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .nest("/api", user_routes)
        .nest("/api", products_routes)
        .nest("/api", role_routes)
        .with_state(db_pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
