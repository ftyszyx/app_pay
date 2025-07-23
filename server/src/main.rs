use crate::types::common::AppState;
use crate::types::config::Config;
use crate::utils::redis_cache::RedisCache;
use chrono::{FixedOffset, Utc};
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;
use std::net::SocketAddr;
use tracing_appender::rolling;
use tracing_subscriber::{fmt::time::FormatTime, layer::SubscriberExt, util::SubscriberInitExt};

pub mod constants;
mod database;
mod handlers;
pub mod my_error;
mod router;
mod types;
mod utils;

struct East8Timer;

impl FormatTime for East8Timer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let east8 = FixedOffset::east_opt(8 * 3600).unwrap();
        let now = Utc::now().with_timezone(&east8);
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // 初始化日志
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "error".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_timer(East8Timer)
                .with_ansi(false)
                .with_writer(non_blocking_appender),
        )
        .with(tracing_subscriber::fmt::layer().with_timer(East8Timer))
        .init();
    // 加载配置
    let config = Config::from_env().unwrap();
    tracing::info!("Configuration loaded successfully");
    // 初始化数据库
    let db_pool = database::init_db(&config.database).await.unwrap();
    tracing::info!("Database connected successfully");
    // 运行数据库迁移
    Migrator::up(&db_pool, None).await.unwrap();
    tracing::info!("Database migration completed");
    // 初始化 Redis
    let redis = RedisCache::new(&config.redis.url).unwrap();
    tracing::info!("Redis connected successfully");
    // 创建应用状态
    let app_state = AppState {
        db: db_pool,
        redis: Arc::new(redis),
        config: Arc::new(config.clone()),
    };
    // 创建路由
    let app = router::create_router(app_state);
    // 启动服务器
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::Ipv4Addr>()
            .unwrap_or_else(|_| std::net::Ipv4Addr::new(127, 0, 0, 1)),
        config.server.port,
    ));
    tracing::info!("Server starting on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
