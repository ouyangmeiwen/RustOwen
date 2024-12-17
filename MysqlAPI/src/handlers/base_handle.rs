use crate::models::claims_model::Claims;
use actix_web::HttpMessage;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::collections::HashMap;

pub async fn check_auth(req: &HttpRequest) -> Result<Claims, actix_web::Error> {
    // 从请求的扩展字段中获取 HashMap
    if let Some(flags) = req.extensions().get::<HashMap<&str, String>>() {
        // if !req.path().contains("token") {
        //     if let Some(auth_failed) = flags.get("auth_failed") {
        //         if auth_failed == "true" {
        //             // 如果认证失败，返回 401 Unauthorized
        //             return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
        //         }
        //     }
        // }
        // if let Some(auth_failed) = flags.get("rate_limit_exceeded") {
        //     if auth_failed == "true" {
        //         // 如果认证失败，返回 401 Unauthorized
        //         return Err(actix_web::error::ErrorTooManyRequests(
        //             "ErrorTooManyRequests",
        //         ));
        //     }
        // }
        let mut claims = Claims::default();
        if let Some(user_id) = flags.get("user_id") {
            claims.user_id = user_id.to_string();
            //println!("user_id:{}", user_id.to_string());
        }
        if let Some(user_role) = flags.get("user_role") {
            claims.role = user_role.to_string();
            //println!("user_role:{}", user_role.to_string());
        }

        return Ok(claims);
    }
    // 如果认证未失败，返回一个空的 HashMap 或其他信息
    Ok(Claims::default())
}
