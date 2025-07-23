use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;
use crate::types::config::DatabaseConfig;

pub async fn init_db(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    tracing::info!("Connecting to database: {}", config.url);
    let mut opt = ConnectOptions::new(&config.url);
    opt.max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect_timeout(Duration::from_secs(config.connect_timeout_secs))
        .sqlx_logging(true);
    let db = Database::connect(opt).await?;
    Ok(db)
}
