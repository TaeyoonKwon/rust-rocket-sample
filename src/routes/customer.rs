use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

use crate::models::customer::Customer;
use crate::models::customer::CustomerInput;
use crate::models::response::MessageResponse;

use crate::db::customer;

/// get customer documents
#[openapi(tag = "Customer")]
#[get("/customer?<limit>&<page>")]
pub async fn get_customers(
    db: &State<Database>,
    limit: i64,
    page: Option<i64>,
) -> Result<Json<Vec<Customer>>, BadRequest<Json<MessageResponse>>> {
    // Error handling
    if limit < 0 {
        return Err(BadRequest(Some(Json(MessageResponse {
            message: "limit cannot be less than 0".to_string(),
        }))));
    }

    if !page.is_none() && page.unwrap() < 1 {
        return Err(BadRequest(Some(Json(MessageResponse {
            message: "page cannot be less than 1".to_string(),
        }))));
    }

    match customer::find_customer(&db, limit, if page.is_none() { 1 } else { page.unwrap() }).await
    {
        Ok(_customer_docs) => Ok(Json(_customer_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            Err(BadRequest(Some(Json(MessageResponse {
                message: _error.to_string(),
            }))))
        }
    }
}

/// get customer document by _id
#[openapi(tag = "Customer")]
#[get("/customer/<_id>")]
pub async fn get_customer_by_id(
    db: &State<Database>,
    _id: String,
) -> Result<Json<Customer>, BadRequest<Json<MessageResponse>>> {
    let oid = ObjectId::parse_str(&_id);

    if oid.is_err() {
        return Err(BadRequest(Some(Json(MessageResponse {
            message: format!("Invalid document id {}", &_id),
        }))));
    }

    match customer::find_customer_by_id(&db, oid.unwrap()).await {
        Ok(_customer_doc) => Ok(Json(_customer_doc)),
        Err(_error) => {
            println!("{:?}", _error);
            Err(BadRequest(Some(Json(MessageResponse {
                message: "No document found".to_string(),
            }))))
        }
    }
}

/// create a customer document
#[openapi(tag = "Customer")]
#[post("/customer", data = "<input>")]
pub async fn post_customer(
    db: &State<Database>,
    input: Json<CustomerInput>,
) -> Option<Json<String>> {
    match customer::insert_customer(&db, input).await {
        Ok(_customer_doc_id) => {
            return Some(Json(_customer_doc_id));
        }
        Err(_error) => {
            println!("{:?}", _error);
            return None;
        }
    }
}

/// update a customer document by _id
#[openapi(tag = "Customer")]
#[patch("/customer/<_id>", data = "<input>")]
pub async fn patch_customer_by_id(
    db: &State<Database>,
    _id: String,
    input: Json<CustomerInput>,
) -> Result<Json<Customer>, BadRequest<Json<MessageResponse>>> {
    let oid = ObjectId::parse_str(&_id);

    if oid.is_err() {
        return Err(BadRequest(Some(Json(MessageResponse {
            message: format!("Invalid document id {}", &_id),
        }))));
    }

    match customer::update_customer_by_id(&db, oid.unwrap(), input).await {
        Ok(_customer_doc) => Ok(Json(_customer_doc)),
        Err(_error) => {
            println!("{:?}", _error);
            Err(BadRequest(Some(Json(MessageResponse {
                message: "No document found".to_string(),
            }))))
        }
    }
}

/// delete a customer document by _id
#[openapi(tag = "Customer")]
#[delete("/customer/<_id>")]
pub async fn delete_customer_by_id(
    db: &State<Database>,
    _id: String,
) -> Result<Json<Customer>, BadRequest<Json<MessageResponse>>> {
    let oid = ObjectId::parse_str(&_id);

    if oid.is_err() {
        return Err(BadRequest(Some(Json(MessageResponse {
            message: format!("Invalid document id {}", &_id),
        }))));
    }

    match customer::delete_customer_by_id(&db, oid.unwrap()).await {
        Ok(_customer_doc) => Ok(Json(_customer_doc)),
        Err(_error) => {
            println!("{:?}", _error);
            Err(BadRequest(Some(Json(MessageResponse {
                message: "No document found".to_string(),
            }))))
        }
    }
}
