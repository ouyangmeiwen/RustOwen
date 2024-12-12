use crate::models::apiresponse_model::ApiResponse;
use crate::models::appstate_model::AppState;
use crate::schemas::rabbitmq_schema::RabbitMQMsgInput;
use crate::RABBITMQ_ROUTING_EXCHANGE;
use actix_web::{get, post, web, HttpResponse, Responder};

//http://127.0.0.1:7788/api/rabbitmq/send?routing_key=routing_key.key&msg=1232131111
#[post("/rabbitmq/send")]
pub async fn sendmsg_rabbitmq_handle(
    body: web::Json<RabbitMQMsgInput>, // 通过请求体接收 user_id
    data: web::Data<AppState>,
) -> impl Responder {
    // 发布者
    match data.rabbitmq.as_ref() {
        Some(rabbitmq) => {
            let exchange = RABBITMQ_ROUTING_EXCHANGE.lock().unwrap();
            match rabbitmq
                .publish(&exchange.clone(), &body.routing_key, &body.msg)
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
                    eprintln!("{}", error_message);
                    HttpResponse::InternalServerError()
                        .json(ApiResponse::<()>::error(&error_message))
                }
            }
        }
        None => {
            // 处理 rabbitmq 为 None 的情况
            let error_message = "RabbitMQ is not initialized";
            eprintln!("{}", error_message);
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(error_message))
        }
    }
}
