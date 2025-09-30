use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use hyper::body::to_bytes;
use hyper::{Body, Request, Response, StatusCode};
use rusqlite::{Connection, params};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

// Initialize SQLite DB
pub fn init_db() -> Connection {
    let conn = Connection::open("auth.db").expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        )",
        [],
    )
    .unwrap();
    conn
}

// Register a new user
pub async fn register_user(conn: &Connection, req: Request<Body>) -> Response<Body> {
    let bytes = to_bytes(req.into_body()).await.unwrap();
    let auth: AuthRequest = match serde_json::from_slice(&bytes) {
        Ok(a) => a,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid JSON"))
                .unwrap();
        }
    };

    // Hash password
    // Generate salt from a UUID
    let salt_string = Uuid::new_v4().to_string(); // e.g., "550e8400-e29b-41d4-a716-446655440000"
    let salt = SaltString::b64_encode(salt_string.as_bytes()).expect("Failed to create salt");
    let password_hash = Argon2::default()
        .hash_password(auth.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    match conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
        params![auth.username, password_hash],
    ) {
        Ok(_) => {
            info!("User '{}' registered", auth.username);
            Response::new(Body::from(format!("User '{}' registered", auth.username)))
        }
        Err(e) => Response::builder()
            .status(StatusCode::CONFLICT)
            .body(Body::from(format!("Error: {}", e)))
            .unwrap(),
    }
}

// Login user
pub async fn login_user(conn: &Connection, req: Request<Body>) -> Response<Body> {
    let bytes = to_bytes(req.into_body()).await.unwrap();
    let auth: AuthRequest = match serde_json::from_slice(&bytes) {
        Ok(a) => a,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid JSON"))
                .unwrap();
        }
    };

    let mut stmt = conn
        .prepare("SELECT password_hash FROM users WHERE username=?1")
        .unwrap();
    let mut rows = stmt.query([&auth.username]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let stored_hash: String = row.get(0).unwrap();
        let parsed_hash = PasswordHash::new(&stored_hash).unwrap();
        if Argon2::default()
            .verify_password(auth.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            info!("User '{}' logged in", auth.username);
            Response::new(Body::from(format!("User '{}' logged in", auth.username)))
        } else {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Invalid password"))
                .unwrap()
        }
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("User not found"))
            .unwrap()
    }
}
