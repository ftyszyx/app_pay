use crate::handlers::{self, middleware::auth};
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
        // app
        handlers::app_handler::add,
        handlers::app_handler::get_list,
        handlers::app_handler::get_by_id,
        handlers::app_handler::update,
        handlers::app_handler::fake_delete,
        //users
        handlers::user_handler::add,
        handlers::user_handler::get_by_id,
        handlers::user_handler::update,
        handlers::user_handler::fake_delete,
        handlers::user_handler::get_list,
        //role
        handlers::role_handler::add,
        handlers::role_handler::get_list,
        handlers::role_handler::get_by_id,
        handlers::role_handler::update,
        handlers::role_handler::fake_delete,
        //products
        handlers::product_handler::add,
        handlers::product_handler::get_list,
        handlers::product_handler::get_by_id,
        handlers::product_handler::update,
        handlers::product_handler::fake_delete,
        //pay_methods
        handlers::pay_method_handler::add,
        handlers::pay_method_handler::get_list,
        handlers::pay_method_handler::get_by_id,
        handlers::pay_method_handler::update,
        handlers::pay_method_handler::fake_delete,
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
        .route("/users", post(handlers::user_handler::add))
        .route("/users/list", post(handlers::user_handler::get_list))
        .route("/users/{id}", get(handlers::user_handler::get_by_id))
        .route("/users/{id}", put(handlers::user_handler::update))
        .route("/users/{id}", delete(handlers::user_handler::fake_delete))
        .route_layer(middleware::from_fn(auth));

    let app_routes = Router::new()
        .route("/apps", post(handlers::app_handler::add))
        .route("/apps/list", get(handlers::app_handler::get_list))
        .route("/apps/{id}", get(handlers::app_handler::get_by_id))
        .route("/apps/{id}", put(handlers::app_handler::update))
        .route("/apps/{id}", delete(handlers::app_handler::fake_delete))
        .route_layer(middleware::from_fn(auth));

    let role_routes = Router::new()
        .route("/roles", post(handlers::role_handler::add))
        .route("/roles/list", get(handlers::role_handler::get_list))
        .route("/roles/{id}", get(handlers::role_handler::get_by_id))
        .route("/roles/{id}", put(handlers::role_handler::update))
        .route("/roles/{id}", delete(handlers::role_handler::fake_delete))
        .route_layer(middleware::from_fn(auth));

    let product_routes = Router::new()
        .route("/products", post(handlers::product_handler::add))
        .route("/products/list", get(handlers::product_handler::get_list))
        .route("/products/{id}", get(handlers::product_handler::get_by_id))
        .route("/products/{id}", put(handlers::product_handler::update))
        .route(
            "/products/{id}",
            delete(handlers::product_handler::fake_delete),
        )
        .route_layer(middleware::from_fn(auth));

    let pay_method_routes = Router::new()
        .route("/pay_methods", post(handlers::pay_method_handler::add))
        .route(
            "/pay_methods/list",
            get(handlers::pay_method_handler::get_list),
        )
        .route(
            "/pay_methods/{id}",
            get(handlers::pay_method_handler::get_by_id),
        )
        .route(
            "/pay_methods/{id}",
            put(handlers::pay_method_handler::update),
        )
        .route(
            "/pay_methods/{id}",
            delete(handlers::pay_method_handler::fake_delete),
        )
        .route_layer(middleware::from_fn(auth));

    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .route("/", get(handlers::handler))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .nest("/api/admin", user_routes)
        .nest("/api/admin", app_routes)
        .nest("/api/admin", role_routes)
        .nest("/api/admin", product_routes)
        .nest("/api/admin", pay_method_routes)
        .with_state(db_pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
