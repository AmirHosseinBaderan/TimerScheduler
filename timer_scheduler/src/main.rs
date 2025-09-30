use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tracing::{info, error};
use tracing_subscriber;

mod router;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber (pretty console logging)
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server starting on http://{}", addr);

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(router::router))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run the server forever
    if let Err(e) = server.await {
        error!("server error: {}", e);
    }
}