use actix_web::{web,post, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use crate::utils::jwt_utils::create_jwt;
use serde_json::json;

/// 请求体结构
#[derive(Deserialize)]
pub struct TokenRequest {
    user_id: String,  // 用户 ID 或邮箱
}

/// 响应结构
#[derive(Serialize)]
struct TokenResponse {
    token: String,
}

/// 生成 JWT 的 Handler
//http://127.0.0.1:7788/api/generate_token
#[post("/token")]
pub async fn generate_token_handler(
    body: web::Json<TokenRequest>, // 通过请求体接收 user_id
) -> impl Responder {
    let user_id = body.user_id.clone(); // 从请求体中提取 user_id
    match create_jwt(user_id) {
        Ok(token) => HttpResponse::Ok().json(TokenResponse { token }),
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Failed to generate token"
        })),
    }
}
