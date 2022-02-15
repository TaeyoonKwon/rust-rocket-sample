use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::models::response::MessageResponse;

pub mod customer;

/// This is a description. <br />You can do simple html <br /> like <b>this<b/>
#[openapi(tag = "Hello World")]
#[get("/")]
pub fn index() -> Json<MessageResponse> {
    Json(MessageResponse {
        message: "Hello World!".to_string(),
    })
}
