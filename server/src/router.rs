use axum::{
    middleware,
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::{handlers, ApiDoc};
use crate::handlers::middleware::auth;

pub fn create_router(db_pool: DatabaseConnection) -> Router {
    let admin_routes = Router::new()
        .route("/products", get(handlers::product::get_products))
        .route_layer(middleware::from_fn(auth));

    let api_routes = Router::new()
        .route("/products", get(handlers::product_handler::get_all_products));

    let cors = CorsLayer::new().allow_origin(Any);
    Router::new()
        .route("/", get(handlers::handler))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .nest("/api/admin", admin_routes)
        .nest("/api", api_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(db_pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
} 