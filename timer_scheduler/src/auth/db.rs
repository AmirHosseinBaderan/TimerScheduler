use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use sqlx::Row;

pub type DbPool = Pool<Sqlite>;

pub async fn init_db() -> DbPool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://./data/app.db")
        .await
        .expect("❌ Failed to connect to SQLite");

    // Create admin table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS admin (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_name TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )
        "#
    )
        .execute(&pool)
        .await
        .expect("❌ Failed to create admin table");

    // Check if table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM admin")
        .fetch_one(&pool)
        .await
        .expect("❌ Failed to count admins");

    if count.0 == 0 {
        sqlx::query("INSERT INTO admin (user_name, password) VALUES (?, ?)")
            .bind("admin")
            .bind("admin")
            .execute(&pool)
            .await
            .expect("❌ Failed to insert default admin");
        println!("✅ Default admin (admin/admin) created");
    }

    pool
}
