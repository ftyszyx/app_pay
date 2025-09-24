use app_server::{app, router};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    // Load .env if present; do not fail when missing (use env vars instead)
    tracing::info!("Server1 booting");
    let _ = dotenvy::dotenv();
    let _guard = app::init_log();
    tracing::info!("Server boot starting");
    let app_state = app::init_app()
        .await
        .unwrap_or_else(|e| panic!("failed to initialize app:{}", e.to_string()));
    let host = app_state.config.server.host.clone();
    let port = app_state.config.server.port;
    let app = router::create_router(app_state);
    // 启动服务器
    let addr = SocketAddr::from((
        host.parse::<std::net::Ipv4Addr>()
            .unwrap_or_else(|_| std::net::Ipv4Addr::new(127, 0, 0, 1)),
        port,
    ));
    tracing::info!("Server starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
