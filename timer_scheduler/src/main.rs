use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing::{info};
use tracing_subscriber;
use hyper::Server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting Timer Scheduler server...");

    run_server().await;
}

async fn run_server() {
    let app = Router::new()
        .route("/", get(|| async {
            info!("Handling / request");
            "Hello, Timer Scheduler!"
        }));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("🚀 Server running at http://{}", addr);

    // Use hyper::Server to run the app
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
