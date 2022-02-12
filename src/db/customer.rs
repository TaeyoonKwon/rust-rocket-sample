use crate::models::customer::Customer;
use crate::models::customer::CustomerDocument;
use crate::models::customer::CustomerInput;

// use chrono::prelude::*;
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::FindOptions;
use mongodb::Database;
use rocket::serde::json::Json;

pub async fn find_customer(
    db: &Database,
    limit: i64,
    page: i64,
) -> mongodb::error::Result<Vec<Customer>> {
    let collection = db.collection::<CustomerDocument>("customer");

    let find_options = FindOptions::builder()
        .limit(limit)
        .skip(u64::try_from((page - 1) * limit).unwrap())
        .build();

    let mut cursor = collection.find(None, find_options).await?;

    let mut customers: Vec<Customer> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;
        let created_at = result.createdAt;
        // transform ObjectId to String
        let customer_json = Customer {
            _id: _id.to_string(),
            name: name.to_string(),
            createdAt: created_at.to_string(),
        };
        customers.push(customer_json);
    }

    Ok(customers)
}

pub async fn find_customer_by_id(db: &Database, oid: ObjectId) -> mongodb::error::Result<Customer> {
    let collection = db.collection::<CustomerDocument>("customer");

    let customer_doc = collection.find_one(doc! {"_id":oid }, None).await?.unwrap();
    // transform ObjectId to String
    let customer_json = Customer {
        _id: customer_doc._id.to_string(),
        name: customer_doc.name.to_string(),
        createdAt: customer_doc.createdAt.to_string(),
    };

    Ok(customer_json)
}

pub async fn insert_customer(
    db: &Database,
    input: Json<CustomerInput>,
) -> mongodb::error::Result<String> {
    let collection = db.collection::<Document>("customer");

    let created_at: DateTime = DateTime::now();

    let insert_one_result = collection
        .insert_one(
            doc! {"name": input.name.clone(), "createdAt": created_at},
            None,
        )
        .await?;

    Ok(insert_one_result.inserted_id.to_string())
}
