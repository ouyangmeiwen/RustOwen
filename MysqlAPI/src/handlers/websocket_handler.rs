use crate::utils::websockethelper::WebSocketHelper;
use actix::prelude::*;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use log::info;

pub async fn websocket_register(
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
