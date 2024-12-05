// src/main.rs

use actix_web::{web, App, HttpServer};
use crate::config::Config;
use crate::routes::libitem_routes::libitem_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .configure(libitem_routes)  // 引入路由配置函数
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
