use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tracing::{info, error};
use tracing_subscriber;
use r2d2_sqlite::SqliteConnectionManager;
use crate::router::router;

mod groups;
mod router;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server starting on http://{}", addr);

    // Clone pool for each service connection
    let make_svc = make_service_fn(move |_conn| {
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                router(req) // Pass pool to router
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run the server forever
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}
