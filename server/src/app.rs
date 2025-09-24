use crate::database;
use crate::services::casbin_service::CasbinService;
use crate::types::config::Config;
use crate::types::{common::AppState, error::AppError};
use crate::utils::redis_cache::RedisCache;
use aliyun_sts::StsClient;
use chrono::{FixedOffset, Utc};
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{fmt::time::FormatTime, layer::SubscriberExt, util::SubscriberInitExt};
struct East8Timer;

impl FormatTime for East8Timer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let east8 = FixedOffset::east_opt(8 * 3600).unwrap();
        let now = Utc::now().with_timezone(&east8);
        write!(w, "{}", now.format("%Y-%m-%d %H:%M:%S%.3f"))
    }
}

pub async fn init_app() -> Result<AppState, AppError> {
    // 加载配置
    let config = Config::from_env()
        .map_err(|e| AppError::Message(format!("config load failed:{}", e.to_string())))?;
    tracing::info!("Configuration loaded successfully");
    // 初始化数据库
    let db_pool = database::init_db(&config.database)
        .await
        .map_err(|e| AppError::Message(format!("database connection failed:{}", e)))?;
    tracing::info!("Database connected successfully");
    // 运行数据库迁移
    Migrator::up(&db_pool, None)
        .await
        .map_err(|e| AppError::Message(format!("migration failed:{}", e)))?;
    tracing::info!("Database migration completed");
    // 初始化 Redis
    let redis = RedisCache::new(&config.redis.url)
        .map_err(|e| AppError::Message(format!("redis connection failed:{}", e)))?;
    tracing::info!("Redis connected successfully");
    let casbin = CasbinService::new(&db_pool)
        .await
        .map_err(|e| AppError::Message(format!("casbin connection failed:{}", e)))?;
    tracing::info!("Casbin connected successfully");
    // 创建应用状态
    let app_state = AppState {
        db: db_pool,
        redis: Arc::new(redis),
        casbin: Arc::new(casbin),
        aliyun_sts: Arc::new(StsClient::new(
            &config.oss.region,
            &config.oss.access_key_id,
            &config.oss.access_key_secret,
        )),
        config: Arc::new(config),
    };
    // 创建路由
    Ok(app_state)
}

pub fn init_log() -> WorkerGuard {
    // 同时输出到文件和 stdout，并保留 guard 确保文件日志 flush
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info".into());
    let fmt_file = tracing_subscriber::fmt::layer()
        .with_timer(East8Timer)
        .with_ansi(false)
        .with_writer(non_blocking_appender);
    let fmt_stdout = tracing_subscriber::fmt::layer().with_timer(East8Timer);
    let _ = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_file)
        .with(fmt_stdout)
        .try_init();
    guard
}
