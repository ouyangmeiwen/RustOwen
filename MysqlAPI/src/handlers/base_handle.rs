use actix_web::{web, HttpRequest, HttpResponse, Result, App, HttpServer};
use actix_web::HttpMessage;
use std::collections::HashMap;


pub async fn check_auth_failed(req: &HttpRequest) -> Result<(), actix_web::Error> {
    // 从请求的扩展字段中获取 HashMap
    if let Some(flags) = req.extensions().get::<HashMap<&str, bool>>() {
        if let Some(auth_failed) = flags.get("auth_failed") {
            if *auth_failed {
                // 如果认证失败，返回 401 Unauthorized
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            }
        }
    }
    Ok(())
}
