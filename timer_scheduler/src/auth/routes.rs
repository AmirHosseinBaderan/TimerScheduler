use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::Row;
use crate::auth::{db::DbPool, token::create_token};

#[derive(Deserialize)]
pub struct LoginRequest {
    user_name: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    form: web::Json<LoginRequest>
) -> impl Responder {
    let row = sqlx::query("SELECT password FROM admin WHERE user_name = ?")
        .bind(&form.user_name)
        .fetch_optional(pool.get_ref())
        .await;

    match row {
        Ok(Some(record)) => {
            let stored_password: String = record.get("password");
            if stored_password == form.password {
                let token = create_token(&form.user_name);
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "✅ Login successful",
                    "token": token
                }))
            } else {
                HttpResponse::Unauthorized().body("❌ Invalid password")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("❌ User not found"),
        Err(_) => HttpResponse::InternalServerError().body("❌ DB error"),
    }
}
