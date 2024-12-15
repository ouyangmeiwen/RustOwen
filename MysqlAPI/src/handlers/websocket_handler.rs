use actix::prelude::*;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use lazy_static::lazy_static;
use log::info;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::apiresponse_model::ApiResponse; // Make sure you have a logger enabled for the logs
                                                   // Define a custom message type for WebSocket messages
pub struct WsMessage(pub String); // Custom message containing the message to send

impl actix::Message for WsMessage {
    type Result = (); // The result type can be `()` because we're just sending a message
}

// Global static variable for storing WebSocket clients
lazy_static! {
    pub static ref CLIENTS: Arc<Mutex<HashMap<String, Addr<WebSocketHelper>>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub struct WebSocketHelper {
    id: String, // Unique client ID for this WebSocket
}

impl WebSocketHelper {
    fn new(id: String) -> Self {
        WebSocketHelper { id }
    }

    // Send a message to this WebSocket client
    fn send_message(&self, message: String) {
        let clients = CLIENTS.lock().unwrap(); // Lock the Mutex
        if let Some(client) = clients.get(&self.id) {
            let _ = client.do_send(WsMessage(message)); // Send the custom message
        }
    }
    fn send_message_to_client(client_id: String, message: String) {
        let clients = CLIENTS.lock().unwrap(); // Lock the Mutex
        if let Some(client) = clients.get(&client_id) {
            let _ = client.do_send(WsMessage(message)); // Send the message to the specific client
        } else {
            println!("Client with id {} not found.", client_id);
        }
    }
    fn broadcast_message(message: String) {
        let clients = CLIENTS.lock().unwrap(); // Lock the Mutex
        for client in clients.values() {
            let _ = client.do_send(WsMessage(message.clone())); // Send the message to all clients
        }
    }
}

impl Actor for WebSocketHelper {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let clients_addr = ctx.address().clone(); // Get the address of the actor (MyWebSocket)

        // Lock the global CLIENTS Mutex to access the HashMap
        let mut clients = CLIENTS.lock().unwrap();

        // Insert the Addr<MyWebSocket> into the HashMap with the client id as the key
        clients.insert(self.id.clone(), clients_addr);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        // Remove the client from the global list when the actor is stopped
        let mut clients = CLIENTS.lock().unwrap();
        clients.remove(&self.id); // Remove the client based on its id
    }
}

// Implement the Handler trait to handle WsMessage
impl Handler<WsMessage> for WebSocketHelper {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        // Send the message to the WebSocket
        ctx.text(msg.0); // Send the message as text
    }
}

// Implement StreamHandler for handling WebSocket messages (using `ws::Message`)
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketHelper {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Handle the received text message
                println!("Received message: {}", text);
                // Optionally, respond with a message
                ctx.text(format!("Echo: {}", text)); // Echo the received message
            }
            Ok(ws::Message::Binary(bin)) => {
                // Handle binary messages if needed
                println!("Received binary message: {:?}", bin);
            }
            Err(e) => {
                // Handle any protocol errors
                println!("WebSocket error: {}", e);
            }
            _ => (),
        }
    }
}

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
