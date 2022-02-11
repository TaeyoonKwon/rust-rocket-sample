use mongodb::bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

use crate::models::customer::Customer;

use crate::db::customer::find_customer;

/// This is a description. <br />You can do simple html <br /> like <b>this<b/>
#[openapi(tag = "Customer")]
#[get("/customer")]
pub async fn get_customers(db: &State<Database>) -> Option<Json<Vec<Customer>>> {
    match find_customer(&db).await {
        Ok(_customer_docs) => {
            return Some(Json(_customer_docs));
        }
        Err(_error) => {
            println!("{:?}", _error);
            return None;
        }
    }
}
