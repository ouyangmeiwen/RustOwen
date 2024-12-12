use crate::models::apiresponse_model::ApiResponse;
use crate::models::appstate_model::AppState;
use crate::schemas::rabbitmq_schema::RabbitMQMsgInput;
use crate::{
    models::claims_model::Claims, models::claims_model::TokenRequest,
    models::claims_model::TokenResponse, utils::jwt_utils::create_jwt,
};
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::Utc;
use serde_json::json;
//http://127.0.0.1:7788/api/rabbitmq/send
#[post("/rabbitmq/send")]
pub async fn sendmsg_rabbitmq_handle(
    body: web::Json<RabbitMQMsgInput>, // 通过请求体接收 user_id
    data: web::Data<AppState>,
) -> impl Responder {
    let exchange = "topic_logs";
    // 发布者
    match data
        .rabbitmq
        .publish(exchange, &body.routing_key, &body.msg)
        .await
    {
        Ok(()) => {
            // 成功处理
            println!("Message published successfully.");
            HttpResponse::Ok().json(ApiResponse::<()>::success(()))
        }
        Err(e) => {
            // 处理错误
            let error_message = format!("Failed to publish message: {}", e);
            eprintln!("Failed to publish message: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&error_message))
        }
    }
}
