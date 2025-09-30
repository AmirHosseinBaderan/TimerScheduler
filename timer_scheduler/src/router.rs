use hyper::{Body, Method, Request, Response, StatusCode};
use tracing::info;

use crate::groups::auth::{init_db, register_user, login_user};

pub async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    info!("Incoming request: {} {}", method, path);

    // Initialize DB (reuse in-memory or real DB in production)
    let conn = init_db();

    let response = match (method, path.as_str()) {
        (Method::POST, "/auth/register") => register_user(&conn, req).await,
        (Method::POST, "/auth/login") => login_user(&conn, req).await,

        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("No handler for {}", path)))
            .unwrap(),
    };

    Ok(response)
}
