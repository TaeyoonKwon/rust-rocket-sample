use futures::stream::TryStreamExt;
use mongodb::Database;
use mongodb::{bson::Document, options::FindOptions};

use crate::models::customer::Customer;

pub async fn find_customer(db: &Database) -> mongodb::error::Result<Vec<Customer>> {
    let collection = db.collection::<Document>("customer");

    let find_options = FindOptions::builder().limit(10).build();

    let mut cursor = collection.find(None, find_options).await?;

    let mut customers: Vec<Customer> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let name = result.get_str("name").unwrap();
        let customer_json = Customer {
            name: name.to_string(),
        };
        customers.push(customer_json);
    }

    Ok(customers)
}
