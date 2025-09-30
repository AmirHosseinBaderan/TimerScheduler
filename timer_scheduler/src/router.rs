use hyper::{Body, Request, Response, Method, StatusCode};
use tracing::info;

// Main router function
pub async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::http::Error> {
    info!("Incoming request: {} {}", req.method(), req.uri());

    let response = match (req.method(), req.uri().path()) {
        // Timers route group
        (&Method::GET, "/timers") => timers_list().await,
        (&Method::POST, "/push") => timers_create().await,
        (&Method::DELETE, "/cancel") => timers_delete().await,

        // Default 404
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap(),
    };

    Ok(response)
}

// Example timer handlers
async fn timers_list() -> Response<Body> {
    info!("Listing timers");
    Response::new(Body::from("Timers list"))
}

async fn timers_create() -> Response<Body> {
    info!("Creating a timer");
    Response::new(Body::from("Timer created"))
}

async fn timers_delete() -> Response<Body> {
    info!("Deleting a timer");
    Response::new(Body::from("Timer deleted"))
}
