use crate::configs::envconfig::STATIC_CONFIG;
use crate::handlers::base_handle::check_auth;
use crate::models::apiresponse_model::ApiResponse;
use crate::models::appstate_model::AppState;
use crate::{
    models::claims_model::Claims, models::claims_model::TokenRequest,
    models::claims_model::TokenResponse, utils::jwt_utils::create_jwt,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
/// 生成 JWT 的 Handler
//http://127.0.0.1:7788/api/token
#[post("/token")]
pub async fn generate_token_handler(
    body: web::Json<TokenRequest>, // 通过请求体接收 user_id
    data: web::Data<AppState>,
    req: HttpRequest, // 接收请求对象作为参数
) -> impl Responder {
    match check_auth(&req).await {
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(&err.to_string()));
        }
        Ok(_) => {}
    }
    let user_id = body.user_id.clone(); // 从请求体中提取 user_id

    let my_claims = Claims {
        user_id: user_id.to_string(),
        exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        ..Default::default()
    };
    match create_jwt(&my_claims) {
        Ok(token) => {
            // 初始化 Redis 客户端
            let redis_client: &crate::models::redisclient_model::RedisClient = &data.redis_client;
            // 确保 `set` 方法返回的是 `Result<(), String>` 类型，否则你需要做额外的错误处理
            redis_client.set(&user_id, &token).await.unwrap_or(());
            redis_client
                .expire(&user_id, (24 * 3600) as usize)
                .await
                .unwrap_or(());
            //return HttpResponse::Ok().json(TokenResponse { token }); //分号则用reutrn，否则不用return
            HttpResponse::Ok().json(TokenResponse { token }) //分号则用reutrn，否则不用return
        }
        // Err(_) => HttpResponse::InternalServerError().json(json!({
        //     "status": "error",
        //     "message": "Failed to generate token"
        // })),
        Err(_) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error("Failed to generate token")),
    }
}
/// 生成 JWT 的 Handler
//http://127.0.0.1:7788/api/tokenget?user_id=123
#[get("/tokenget")]
pub async fn generate_token_get_handler(
    query: web::Query<TokenRequest>, // 通过请求体接收 user_id
    data: web::Data<AppState>,
    req: HttpRequest, // 接收请求对象作为参数
) -> impl Responder {
    match check_auth(&req).await {
        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(&err.to_string()));
        }
        Ok(_) => {}
    }

    let user_id = query.user_id.clone(); // 从请求体中提取 user_id

    let config = STATIC_CONFIG.read().unwrap(); //智能指针
    println!("Database URL: {}", config.database_url);
    println!("Port: {}", config.port);
    println!("Log Level: {}", config.log_level);

    let my_claims = Claims {
        user_id: user_id.clone(),
        exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        ..Default::default()
    };
    match create_jwt(&my_claims) {
        Ok(token) => {
            // 初始化 Redis 客户端
            let redis_client = &data.redis_client;
            // 确保 `set` 方法返回的是 `Result<(), String>` 类型，否则你需要做额外的错误处理
            redis_client.set(&user_id, &token).await.unwrap_or(());
            redis_client
                .expire(&user_id, (24 * 3600) as usize)
                .await
                .unwrap_or(());
            //return HttpResponse::Ok().json(TokenResponse { token }); //分号则用reutrn，否则不用return
            HttpResponse::Ok().json(TokenResponse { token }) //分号则用reutrn，否则不用return
        }
        // Err(_) => HttpResponse::InternalServerError().json(json!({
        //     "status": "error",
        //     "message": "Failed to generate token"
        // })),
        Err(_) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error("Failed to generate token")),
    }
}
