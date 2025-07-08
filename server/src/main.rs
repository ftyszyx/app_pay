use axum::{
    Router,
    response::Html,
    routing::{get, post},
};
use migration::{Migrator, MigratorTrait};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod database;
mod entities;
mod handlers;

#[tokio::main]
async fn main() {
    let db_pool = database::init_db()
        .await
        .expect("Database connection failed");
    Migrator::up(&db_pool, None).await.unwrap();
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(handler))
        .route("/api/products", get(handlers::product::get_products))
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        .with_state(db_pool)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
