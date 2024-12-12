use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RabbitMQMsgInput {
    pub msg: String,
    pub routing_key: String,
}
