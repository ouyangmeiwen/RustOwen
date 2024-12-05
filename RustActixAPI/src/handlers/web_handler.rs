use crate::handlers::note_handler; // 引用 handler 模块

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

//http://127.0.0.1:7788/api/healthchecker
#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(note_handler::note_list_handler)
        .service(note_handler::create_note_handler)
        .service(note_handler::get_note_handler)
        .service(note_handler::edit_note_handler)
        .service(note_handler::delete_note_handler);

    conf.service(scope);
}
