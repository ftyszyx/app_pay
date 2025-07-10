use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await?;
    Ok(db)
}
