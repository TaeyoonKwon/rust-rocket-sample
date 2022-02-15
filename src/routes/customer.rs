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

use crate::request_guards::basic::ApiKey;

use crate::db::customer;

use crate::errors::response::MyError;

/// get customer documents
#[openapi(tag = "Customer")]
#[get("/customer?<limit>&<page>")]
pub async fn get_customers(
    db: &State<Database>,
    limit: Option<i64>,
    page: Option<i64>,
) -> Result<Json<Vec<Customer>>, MyError> {
    // Error handling
    // This is also valid when strict checking is necessary.
    // if limit < 0 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "limit cannot be less than 0".to_string(),
    //     }))));
    // }
    // if !page.is_none() && page.unwrap() < 1 {
    //     return Err(BadRequest(Some(Json(MessageResponse {
    //         message: "page cannot be less than 1".to_string(),
    //     }))));
    // }

    // Setting default values
    let limit: i64 = limit.unwrap_or(12);
    let page: i64 = page.unwrap_or(1);
    match customer::find_customer(&db, limit, page).await {
        Ok(_customer_docs) => Ok(Json(_customer_docs)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(400, Some(_error.to_string())));
        }
    }
}

/// get customer document by _id
#[openapi(tag = "Customer")]
#[get("/customer/<_id>")]
pub async fn get_customer_by_id(
    db: &State<Database>,
    _id: String,
) -> Result<Json<Customer>, MyError> {
    let oid = ObjectId::parse_str(&_id);

    if oid.is_err() {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    }

    match customer::find_customer_by_id(&db, oid.unwrap()).await {
        Ok(_customer_doc) => {
            if _customer_doc.is_none() {
                return Err(MyError::build(
                    400,
                    Some(format!("Customer not found with _id {}", &_id)),
                ));
            }
            Ok(Json(_customer_doc.unwrap()))
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {}", &_id)),
            ));
        }
    }
}

/// create a customer document
#[openapi(tag = "Customer")]
#[post("/customer", data = "<input>")]
pub async fn post_customer(
    db: &State<Database>,
    input: Json<CustomerInput>,
) -> Result<Json<String>, BadRequest<Json<MessageResponse>>> {
    // can set with a single error like this.
    match customer::insert_customer(&db, input).await {
        Ok(_customer_doc_id) => {
            return Ok(Json(_customer_doc_id));
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(BadRequest(Some(Json(MessageResponse {
                message: format!("Invalid input"),
            }))));
        }
    }
}

/// update a customer document by _id
#[openapi(tag = "Customer")]
#[patch("/customer/<_id>", data = "<input>")]
pub async fn patch_customer_by_id(
    db: &State<Database>,
    _key: ApiKey,
    _id: String,
    input: Json<CustomerInput>,
) -> Result<Json<Customer>, MyError> {
    let oid = ObjectId::parse_str(&_id);

    if oid.is_err() {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    }

    match customer::update_customer_by_id(&db, oid.unwrap(), input).await {
        Ok(_customer_doc) => {
            if _customer_doc.is_none() {
                return Err(MyError::build(
                    400,
                    Some(format!("Customer not found with _id {}", &_id)),
                ));
            }
            Ok(Json(_customer_doc.unwrap()))
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {}", &_id)),
            ));
        }
    }
}

/// delete a customer document by _id
#[openapi(tag = "Customer")]
#[delete("/customer/<_id>")]
pub async fn delete_customer_by_id(
    db: &State<Database>,
    _id: String,
    _key: ApiKey,
) -> Result<Json<Customer>, MyError> {
    let oid = ObjectId::parse_str(&_id);

    if oid.is_err() {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    }

    match customer::delete_customer_by_id(&db, oid.unwrap()).await {
        Ok(_customer_doc) => {
            if _customer_doc.is_none() {
                return Err(MyError::build(
                    400,
                    Some(format!("Customer not found with _id {}", &_id)),
                ));
            }
            Ok(Json(_customer_doc.unwrap()))
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {}", &_id)),
            ));
        }
    }
}
