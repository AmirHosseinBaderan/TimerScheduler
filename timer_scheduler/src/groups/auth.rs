use rusqlite::{params, Connection};
use hyper::{Body, Response, Request, StatusCode};
use hyper::body::to_bytes;
use serde::Deserialize;
use tracing::info;
use argon2::{Argon2, password_hash::{SaltString, PasswordHasher, PasswordVerifier, PasswordHash} };
use rand_core::OsRng;

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
    ).unwrap();
    conn
}

// Register a new user
pub async fn register_user(conn: &Connection, req: Request<Body>) -> Response<Body> {
    let bytes = to_bytes(req.into_body()).await.unwrap();
    let auth: AuthRequest = match serde_json::from_slice(&bytes) {
        Ok(a) => a,
        Err(_) => return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid JSON"))
            .unwrap(),
    };

    // Hash password
    let salt = SaltString::generate(&mut OsRng);
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
        },
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
        Err(_) => return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid JSON"))
            .unwrap(),
    };

    let mut stmt = conn.prepare("SELECT password_hash FROM users WHERE username=?1").unwrap();
    let mut rows = stmt.query([auth.username]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let stored_hash: String = row.get(0).unwrap();
        let parsed_hash = PasswordHash::new(&stored_hash).unwrap();
        if Argon2::default().verify_password(auth.password.as_bytes(), &parsed_hash).is_ok() {
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
