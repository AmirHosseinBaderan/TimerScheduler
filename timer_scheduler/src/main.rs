mod app;
mod auth;
mod ws;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use app::{db as app_db, routes as app_routes};
use auth::{db, routes};
use crate::ws::ws_index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = auth::db::init_db().await;

    // init tables for both modules
    auth::db::init_db().await;
    app_db::init_db().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            // auth
            .service(auth::routes::login)
            // app
            .service(app_routes::create)
            .service(app_routes::delete)
            .service(app_routes::get_list)
            .route("/ws", web::get().to(ws_index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
