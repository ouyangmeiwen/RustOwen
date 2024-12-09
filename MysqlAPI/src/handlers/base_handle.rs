use actix_web::HttpMessage;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::collections::HashMap;

pub async fn check_auth(req: &HttpRequest) -> Result<HashMap<&str, String>, actix_web::Error> {
    // 从请求的扩展字段中获取 HashMap
    if let Some(flags) = req.extensions().get::<HashMap<&str, String>>() {
        if let Some(auth_failed) = flags.get("auth_failed") {
            if auth_failed.to_string() == "true" {
                // 如果认证失败，返回 401 Unauthorized
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
            }
        }
        let mut response_map = HashMap::new();
        if let Some(user_id) = flags.get("user_id") {
            response_map.insert("user_id", user_id.to_string());
        }
        if let Some(user_role) = flags.get("user_role") {
            response_map.insert("user_role", user_role.to_string());
        }
        // 返回 HashMap 而不是错误
        return Ok(response_map);
    }
    // 如果认证未失败，返回一个空的 HashMap 或其他信息
    Ok(HashMap::new())
}
