use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use sqlx::Row;

pub type DbPool = Pool<Sqlite>;

#[derive(Serialize)]
pub struct AppItem {
    pub id: i64,
    pub name: String,
    pub token: String,
}

pub async fn init_db() -> DbPool {
    // Connect to the SQLite database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://./data/app.db")
        .await
        .expect("❌ Failed to connect to SQLite");

    // Create app table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS app (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            token TEXT NOT NULL
        )
        "#
    )
        .execute(&pool)
        .await
        .expect("❌ Failed to create app table");

    pool
}

pub async fn create_app(pool: &DbPool, name: &str) -> sqlx::Result<i64> {
    // generate random base64-like token
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let res = sqlx::query("INSERT INTO app (name, token) VALUES (?, ?)")
        .bind(name)
        .bind(token)
        .execute(pool)
        .await?;

    Ok(res.last_insert_rowid())
}

pub async fn delete_app(pool: &DbPool, id: i64) -> sqlx::Result<u64> {
    let res = sqlx::query("DELETE FROM app WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(res.rows_affected())
}

pub async fn list_apps(pool: &Pool<Sqlite>) -> sqlx::Result<Vec<AppItem>> {
    let rows = sqlx::query("SELECT id, name, token FROM app")
        .fetch_all(pool)
        .await?;

    let apps = rows.into_iter().map(|row| AppItem {
        id: row.get("id"),
        name: row.get("name"),
        token: row.get("token"),
    }).collect();

    Ok(apps)
}