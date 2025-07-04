use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase};

const DB_URL: &str = "sqlite:db.sqlite3";

pub async fn init_db() -> SqlitePool {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS products (
        id INTEGER PRIMARY KEY NOT NULL,
        name VARCHAR(250) NOT NULL,
        price INTEGER NOT NULL,
        description TEXT
    );",
    )
    .execute(&db)
    .await
    .unwrap();

    println!("Create products table result: {:?}", result);

    // Insert a sample product
    let product_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
        .fetch_one(&db)
        .await
        .unwrap();

    if product_count.0 == 0 {
        println!("Inserting sample product...");
        let insert_result =
            sqlx::query("INSERT INTO products (name, price, description) VALUES (?, ?, ?)")
                .bind("My Awesome Software")
                .bind(1999) // Price in cents
                .bind("A lifetime license for my awesome software.")
                .execute(&db)
                .await
                .unwrap();
        println!("Sample product inserted: {:?}", insert_result);
    }

    db
}
