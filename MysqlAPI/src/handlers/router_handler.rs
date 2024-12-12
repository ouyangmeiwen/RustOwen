use crate::handlers::auth_handler;
use crate::handlers::libitem_handler;
use crate::handlers::note_handler; // 引用 handler 模块 // 引用 handler 模块

use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

//http://127.0.0.1:7788/api/healthchecker
#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Service is Running!"; //const 关键字用于定义不可变的编译时常量。这些常量在编译时就已经确定值
    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        //health_checker
        .service(health_checker_handler)
        //auth_handler
        .service(auth_handler::generate_token_handler)
        .service(auth_handler::generate_token_get_handler)
        //note
        .service(note_handler::note_list_handler)
        .service(note_handler::create_note_handler)
        .service(note_handler::get_note_handler)
        .service(note_handler::edit_note_handler)
        .service(note_handler::delete_note_handler)
        // 为 libitem 添加路由
        .service(libitem_handler::libitem_list_handler)
        .service(libitem_handler::create_libitem_handler)
        .service(libitem_handler::get_libitem_handler)
        .service(libitem_handler::get_item_bybarcode_handle)
        .service(libitem_handler::edit_libitem_handler)
        .service(libitem_handler::delete_libitem_handler);

    conf.service(scope);
}
