#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod db;
mod models;
mod routes;

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/openapi.json".to_string(),
        ..Default::default()
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::init().await)
        .mount(
            "/",
            openapi_get_routes![
                routes::index,
                routes::customer::get_customers,
                routes::customer::get_customer_by_id,
                routes::customer::post_customer
            ],
        )
        .mount("/api-docs", make_swagger_ui(&get_docs()))
}
