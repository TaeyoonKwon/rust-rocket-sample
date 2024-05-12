use mongodb::{
    bson::{doc, oid::ObjectId},
    Database,
};
use rocket::{response::status::BadRequest, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    db::customer,
    errors::response::MyError,
    models::{
        customer::{Customer, CustomerInput},
        response::MessageResponse,
    },
    request_guards::basic::ApiKey,
};

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
    match customer::find_customer(db, limit, page).await {
        Ok(_customer_docs) => Ok(Json(_customer_docs)),
        Err(_error) => {
            println!("{:?}", _error);

            Err(MyError::build(400, Some(_error.to_string())))
        }
    }
}

/// get customer document by _id
#[openapi(tag = "Customer")]
#[get("/customer/<id>")]
pub async fn get_customer_by_id(db: &State<Database>, id: &str) -> Result<Json<Customer>, MyError> {
    let oid = ObjectId::parse_str(id);

    if oid.is_err() {
        return Err(MyError::build(400, Some("Invalid _id format.".to_string())));
    }

    match customer::find_customer_by_id(db, oid.unwrap()).await {
        Ok(customer_doc) => {
            if customer_doc.is_none() {
                return Err(MyError::build(
                    400,
                    Some(format!("Customer not found with _id {}", &id)),
                ));
            }
            Ok(Json(customer_doc.unwrap()))
        }
        Err(error) => {
            println!("{:?}", error);

            Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {}", &id)),
            ))
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
    match customer::insert_customer(db, input).await {
        Ok(customer_doc_id) => Ok(Json(customer_doc_id)),
        Err(error) => {
            println!("{:?}", error);
            Err(BadRequest(Json(MessageResponse {
                message: "Invalid input".to_string(),
            })))
        }
    }
}

/// update a customer document by _id
#[openapi(tag = "Customer")]
#[patch("/customer/<id>", data = "<input>")]
pub async fn patch_customer_by_id(
    db: &State<Database>,
    _key: ApiKey,
    id: &str,
    input: Json<CustomerInput>,
) -> Result<Json<Customer>, MyError> {
    let oid = ObjectId::parse_str(id);

    if oid.is_err() {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    }

    match customer::update_customer_by_id(db, oid.unwrap(), input).await {
        Ok(customer_doc) => {
            if customer_doc.is_none() {
                return Err(MyError::build(
                    400,
                    Some(format!("Customer not found with id {}", &id)),
                ));
            }
            Ok(Json(customer_doc.unwrap()))
        }
        Err(error) => {
            println!("{:?}", error);
            Err(MyError::build(
                400,
                Some(format!("Customer not found with id {}", &id)),
            ))
        }
    }
}

/// delete a customer document by _id
#[openapi(tag = "Customer")]
#[delete("/customer/<id>")]
pub async fn delete_customer_by_id(
    db: &State<Database>,
    id: &str,
    _key: ApiKey,
) -> Result<Json<Customer>, MyError> {
    let oid = ObjectId::parse_str(id);

    if oid.is_err() {
        return Err(MyError::build(400, Some("Invalid id format.".to_string())));
    }

    match customer::delete_customer_by_id(db, oid.unwrap()).await {
        Ok(customer_doc) => {
            if customer_doc.is_none() {
                return Err(MyError::build(
                    400,
                    Some(format!("Customer not found with id {}", &id)),
                ));
            }
            Ok(Json(customer_doc.unwrap()))
        }
        Err(error) => {
            println!("{:?}", error);
            Err(MyError::build(
                400,
                Some(format!("Customer not found with _id {}", &id)),
            ))
        }
    }
}
