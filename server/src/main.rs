use std::net::SocketAddr;
use app_server::{app, router,types::common::AppState};
#[tokio::main]
async fn main() {
    let app_state = app::init_app().await.unwrap();
    let app = router::create_router(app_state);
    // 启动服务器
    let addr = SocketAddr::from((
        app_state.config.clone().server.host.parse::<std::net::Ipv4Addr>()
            .unwrap_or_else(|_| std::net::Ipv4Addr::new(127, 0, 0, 1)),
        app_state.config.clone().server.port,
    ));
    tracing::info!("Server starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
