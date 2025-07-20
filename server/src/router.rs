use axum::{
    middleware,
    Router,
    routing::{get, post, put, delete},
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::{handlers, ApiDoc};
use crate::handlers::middleware::auth;

pub fn create_router(db_pool: DatabaseConnection) -> Router {
    let api_authed_routes = Router::new()
        .route("/me", get(handlers::auth::get_current_user))
        .route("/admin/products", get(handlers::product::get_products))
        .route("/admin/users", post(handlers::user_handler::create_user))
        .route("/admin/users", get(handlers::user_handler::get_users_list))
        .route("/admin/users/:id", get(handlers::user_handler::get_user_by_id))
        .route("/admin/users/:id", put(handlers::user_handler::update_user))
        .route("/admin/users/:id", delete(handlers::user_handler::delete_user))
        .route_layer(middleware::from_fn(auth));

    let api_routes = Router::new()
        .route("/products", get(handlers::product_handler::get_all_products));

    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .route("/", get(handlers::handler))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .nest("/api", api_authed_routes)
        .nest("/api", api_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(db_pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
} 