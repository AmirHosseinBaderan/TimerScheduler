mod auth;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use auth::{db, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::init_db().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(routes::login) // POST /login
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
