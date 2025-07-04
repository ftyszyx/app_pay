use axum::{Router, response::Html, routing::get};
use migration::{Migrator, MigratorTrait};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod database;
mod entities;
mod handlers;

#[tokio::main]
async fn main() {
    let db_pool = database::init_db().await;
    Migrator::up(&db_pool, None).await.unwrap();

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", get(handler))
        .route("/api/products", get(handlers::get_products))
        .with_state(db_pool)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
