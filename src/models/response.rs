use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Responder, Debug, Deserialize, Serialize, JsonSchema)]
#[response(status = 400, content_type = "json")]
pub struct MessageResponse {
    /// This is a message from the server.
    pub message: String,
}
