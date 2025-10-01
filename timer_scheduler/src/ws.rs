use actix::prelude::*;
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use serde::Deserialize;
use sqlx::Pool;
use sqlx::Sqlite;

#[derive(Deserialize)]
pub struct TimerMessage {
    pub key: String,
    pub duration_secs: u64,
}

// WebSocket actor
pub struct TimerWs {
    pub app_name: String,
    pub db_pool: Pool<Sqlite>,
}

impl Actor for TimerWs {
    type Context = ws::WebsocketContext<Self>;
}

// Handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TimerWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(timer) = serde_json::from_str::<TimerMessage>(&text) {
                    println!("Received timer from app {}: key={}, duration={}", self.app_name, timer.key, timer.duration_secs);

                    // You can now store this timer in DB or process it
                    // Example: ctx.text("Timer received");
                    ctx.text(format!("Timer {} for {} seconds received", timer.key, timer.duration_secs));
                } else {
                    ctx.text("Invalid timer format");
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket closed for app {}: {:?}", self.app_name, reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

// Entry point for WS handshake
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    db_pool: web::Data<Pool<Sqlite>>,
) -> Result<HttpResponse, Error> {
    // Extract app from header
    let app_name = if let Some(val) = req.headers().get("x-app-name") {
        val.to_str().unwrap_or("unknown").to_string()
    } else {
        "unknown".to_string()
    };

    println!("New WS connection for app: {}", app_name);

    ws::start(TimerWs { app_name, db_pool: db_pool.get_ref().clone() }, &req, stream)
}
