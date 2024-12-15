use crate::utils::websockethelper::WebSocketHelper;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use log::info;

//ws://127.0.0.1:7788/ws/1123123   postman
pub async fn websocket_register_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    // Try to get the client_id from the path
    let client_id = req
        .match_info()
        .get("client_id")
        .map(|id| id.to_string())
        .ok_or_else(|| {
            info!("Missing client_id in the URL"); // Log the error if client_id is missing
            ErrorBadRequest("Missing client_id in the URL")
        })?;

    // Log the client_id to verify it's correct
    info!(
        "WebSocket connection requested with client_id: {}",
        client_id
    );

    // Start the WebSocket session
    ws::start(WebSocketHelper::new(client_id), &req, stream).map_err(|e| {
        info!("Failed to start WebSocket: {}", e); // Log the error if WebSocket startup fails
        ErrorInternalServerError(format!("WebSocket error: {}", e))
    })
}
// HTTP handler for sending a message to a WebSocket client

//http://127.0.0.1:7788/api/websocket/sendmsg/11231231
#[post("/websocket/sendmsg/{client_id}")]
async fn send_message_to_websocket_handler(
    client_id: web::Path<String>, // Client ID is passed as part of the URL path
    message: web::Json<String>,   // The message to send is passed as JSON in the body
) -> impl Responder {
    let client_id = client_id.into_inner(); // Extract client ID from URL path
    let message = message.into_inner(); // Extract message from the request body
                                        // Send the message to the specified client
    if client_id.clone() == "all" {
        WebSocketHelper::broadcast_message(message);
    } else {
        WebSocketHelper::send_message_to_client(client_id.clone(), message);
    }
    HttpResponse::Ok().body(format!("Message sent to WebSocket client: {}", client_id))
}
