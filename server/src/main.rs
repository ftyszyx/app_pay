use axum::{
    Router, middleware,
    response::Html,
    routing::{get, post},
};
use chrono::{FixedOffset, Utc};
use migration::{Migrator, MigratorTrait};
use std::{env, net::SocketAddr};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_appender::rolling;
use tracing_subscriber::{fmt::time::FormatTime, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::middleware::auth;

mod database;
mod entities;
mod handlers;

struct East8Timer;

impl FormatTime for East8Timer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let east8 = FixedOffset::east_opt(8 * 3600).unwrap();
        let now = Utc::now().with_timezone(&east8);
        write!(w, "{}", now.format("%Y-%m-%dT%H:%M:%S"))
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::product::get_products,
    ),
    components(
        schemas(handlers::auth::AuthPayload, handlers::auth::AuthResponse, entities::product::Model)
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "app-pay", description = "App Pay API")
    )
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

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "app_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_timer(East8Timer))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_timer(East8Timer)
                .with_writer(non_blocking_appender),
        )
        .init();

    let db_pool = database::init_db()
        .await
        .expect("Database connection failed");
    Migrator::up(&db_pool, None).await.unwrap();
    let cors = CorsLayer::new().allow_origin(Any);

    let admin_routes = Router::new()
        .route("/products", get(handlers::product::get_products))
        .route_layer(middleware::from_fn(auth));

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .nest("/api/admin", admin_routes)
        // add swagger ui
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(db_pool)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listen_port = env::var("LISTEN_PORT").unwrap().parse().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], listen_port));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
