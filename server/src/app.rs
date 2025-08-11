use crate::database;
use crate::services::casbin_service::CasbinService;
use crate::types::config::Config;
use crate::types::{common::AppState, error::AppError};
use crate::utils::redis_cache::RedisCache;
use aliyun_sts::StsClient;
use chrono::{FixedOffset, Utc};
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;
use tracing_appender::rolling;
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
    dotenvy::dotenv().map_err(|e| AppError::Message(e.to_string()))?;
    init_log();
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

pub fn init_log() {
    // 初始化日志
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);
    let result = tracing_subscriber::registry()
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
        .try_init();
    if let Err(e) = result {
        tracing::warn!("Failed to set global default subscriber: {}", e);
    }
}
