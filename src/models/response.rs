use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct MessageResponse {
    /// This is a message from the server.
    pub message: String,
}
