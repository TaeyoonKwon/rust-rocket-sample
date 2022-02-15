#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

mod db;
mod errors;
mod fairings;
mod models;
mod request_guards;
mod routes;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .attach(db::init())
        .attach(fairings::cors::CORS)
        .mount(
            "/",
            openapi_get_routes![
                routes::index,
                routes::customer::get_customers,
                routes::customer::get_customer_by_id,
                routes::customer::post_customer,
                routes::customer::patch_customer_by_id,
                routes::customer::delete_customer_by_id
            ],
        )
        .mount(
            "/api-docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}

// Unit testings
#[cfg(test)]
mod tests;
