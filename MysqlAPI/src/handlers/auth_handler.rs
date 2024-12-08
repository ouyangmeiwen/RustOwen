use crate::{
    models::claims_model::Claims, models::claims_model::TokenRequest,
    models::claims_model::TokenResponse, utils::jwt_utils::create_jwt,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;

/// 生成 JWT 的 Handler
//http://127.0.0.1:7788/api/token
#[post("/token")]
pub async fn generate_token_handler(
    body: web::Json<TokenRequest>, // 通过请求体接收 user_id
) -> impl Responder {
    let user_id = body.user_id.clone(); // 从请求体中提取 user_id

    let my_claims = Claims {
        user_id: user_id,
        exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        ..Default::default()
    };
    match create_jwt(&my_claims) {
        Ok(token) => HttpResponse::Ok().json(TokenResponse { token }),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to generate token"
        })),
    }
}
/// 生成 JWT 的 Handler
//http://127.0.0.1:7788/api/tokenget?user_id=123
#[get("/tokenget")]
pub async fn generate_token_get_handler(
    body: web::Query<TokenRequest>, // 通过请求体接收 user_id
) -> impl Responder {
    let user_id = body.user_id.clone(); // 从请求体中提取 user_id

    let my_claims = Claims {
        user_id: user_id,
        exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        ..Default::default()
    };
    match create_jwt(&my_claims) {
        Ok(token) => HttpResponse::Ok().json(TokenResponse { token }),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to generate token"
        })),
    }
}
