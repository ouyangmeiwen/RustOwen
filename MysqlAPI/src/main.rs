pub mod handlers;
pub mod models;
pub mod schemas;
pub mod test;

use crate::handlers::web_handler;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool}; // 使用 MySql // 引用 handler 模块
use test::rusttest;

pub struct AppState {
    db: Pool<MySql>, // 将 Pool<Postgres> 改为 Pool<MySql>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rusttest::runtest();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port: u16 = std::env::var("PORT")
        .expect("PORT environment variable must be set")
        .parse()
        .expect("PORT must be a valid u16");

    let pool = match MySqlPoolOptions::new() // 使用 MySqlPoolOptions
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("start http service:http://127.0.0.1:{}", port);  // 使用 format! 或 {} 来插入变量

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(web_handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .shutdown_timeout(30) // 设置优雅关闭的超时，单位是秒
    .run()
    .await
}
