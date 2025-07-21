use chrono::{FixedOffset, Utc};
use migration::{Migrator, MigratorTrait};
use std::{env, net::SocketAddr};
use tracing_appender::rolling;
use tracing_subscriber::{fmt::time::FormatTime, layer::SubscriberExt, util::SubscriberInitExt};

pub mod constants;
mod database;
mod handlers;
pub mod my_error;
mod router;
mod types;
// mod my_macro;

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

    let db_pool = database::init_db()
        .await
        .expect("Database connection failed");
    Migrator::up(&db_pool, None).await.unwrap();

    let app = router::create_router(db_pool);
    let listen_port = env::var("LISTEN_PORT").unwrap().parse().unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], listen_port));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
