// src/routes/libitem_routes.rs

use actix_web::{web, App, Scope};
use crate::services::libitem_service::{create_libitem, get_libitems, get_libitem_by_id, update_libitem, delete_libitem};
use crate::models::libitem::{NewLibItem, UpdateLibItem};

// 创建新的 LibItem
pub async fn create_libitem_handler(
    item: web::Json<NewLibItem>,
    config: web::Data<Config>,
) -> impl Responder {
    let conn = establish_connection(&config);
    let created_item = create_libitem(&conn, item.into_inner());
    HttpResponse::Created().json(created_item)
}

// 获取分页查询的 LibItems
pub async fn get_libitems_handler(
    web::Query(params): web::Query<HashMap<String, String>>,
    config: web::Data<Config>,
) -> impl Responder {
    let conn = establish_connection(&config);
    let page = params.get("page").unwrap_or(&"0".to_string()).parse::<i64>().unwrap();
    let page_size = params.get("page_size").unwrap_or(&"10".to_string()).parse::<i64>().unwrap();
    let items = get_libitems(&conn, page, page_size);
    HttpResponse::Ok().json(items)
}

// 根据 ID 获取单条 LibItem
pub async fn get_libitem_by_id_handler(
    web::Path(item_id): web::Path<String>,
    config: web::Data<Config>,
) -> impl Responder {
    let conn = establish_connection(&config);
    match get_libitem_by_id(&conn, &item_id) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body("Item not found"),
    }
}

// 更新 LibItem
pub async fn update_libitem_handler(
    web::Path(item_id): web::Path<String>,
    item: web::Json<UpdateLibItem>,
    config: web::Data<Config>,
) -> impl Responder {
    let conn = establish_connection(&config);
    let updated_item = update_libitem(&conn, &item_id, item.into_inner());
    HttpResponse::Ok().json(updated_item)
}

// 删除 LibItem
pub async fn delete_libitem_handler(
    web::Path(item_id): web::Path<String>,
    config: web::Data<Config>,
) -> impl Responder {
    let conn = establish_connection(&config);
    delete_libitem(&conn, &item_id);
    HttpResponse::NoContent().finish()
}

// 路由注册函数
pub fn libitem_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/libitems")
            .route(web::post().to(create_libitem_handler))
            .route(web::get().to(get_libitems_handler)),
    )
    .service(
        web::resource("/libitems/{id}")
            .route(web::get().to(get_libitem_by_id_handler))
            .route(web::put().to(update_libitem_handler))
            .route(web::delete().to(delete_libitem_handler)),
    );
}
