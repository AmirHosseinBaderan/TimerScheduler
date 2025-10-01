use actix_web::{post, delete, web, HttpResponse, Responder, get};
use serde::Deserialize;
use crate::app::db::{DbPool, create_app, delete_app, list_apps};

#[derive(Deserialize)]
pub struct CreateAppRequest {
    name: String,
}

#[post("/app")]
pub async fn create(
    pool: web::Data<DbPool>,
    req: web::Json<CreateAppRequest>
) -> impl Responder {
    match create_app(pool.get_ref(), &req.name).await {
        Ok(id) => HttpResponse::Ok().json(serde_json::json!({
            "message": "✅ App created",
            "id": id
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Failed: {e}")),
    }
}

#[delete("/app/{id}")]
pub async fn delete(
    pool: web::Data<DbPool>,
    path: web::Path<i64>
) -> impl Responder {
    let id = path.into_inner();
    match delete_app(pool.get_ref(), id).await {
        Ok(affected) if affected > 0 => HttpResponse::Ok().body("✅ App deleted"),
        Ok(_) => HttpResponse::NotFound().body("❌ App not found"),
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Failed: {e}")),
    }
}

#[get("/apps")]
pub async fn get_list(pool: web::Data<DbPool>) -> impl Responder {
    match list_apps(pool.get_ref()).await {
        Ok(apps) => HttpResponse::Ok().json(apps),
        Err(e) => HttpResponse::InternalServerError().body(format!("❌ Failed: {e}")),
    }
}
