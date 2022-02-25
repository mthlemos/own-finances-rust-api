use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use tokio_compat_02::FutureExt;
use wither::bson::{doc, oid::ObjectId, Document};
use wither::mongodb::Database;
use wither::prelude::Model;
use wither::WitherError;

// Create find wrapper
// In order to make code cleaner, since there is a ton of things needed to only run a single find
#[async_trait::async_trait]
pub trait FindWrapper<T>
where
    // Generic type should be a model
    // The 'Send' part makes it thread-safe
    T: Model + std::marker::Send,
{
    async fn find_wrapper(db: &Database, filter: Option<Document>) -> Result<Vec<T>, WitherError> {
        // Create result vector
        let mut result_vec: Vec<T> = Vec::new();
        let mut cursor = T::find(&db, filter, None).compat().await?;
        // Iterate over found objects
        while let Some(Ok(invoice)) = cursor.next().compat().await {
            result_vec.push(invoice);
        }
        // Return result vector
        Ok(result_vec)
    }
}

#[derive(Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"id": 1}"#))]
pub struct Invoice {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub purchase_date: i32,
    pub billing_type: String,
    pub installments: i8,
    pub recurring: bool,
    pub category: String,
}
impl FindWrapper<Invoice> for Invoice {}

#[derive(Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"id": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}
impl FindWrapper<Category> for Category {}

#[derive(Model, Serialize, Deserialize)]
#[model(index(keys = r#"doc!{"id": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct BillingType {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}
impl FindWrapper<BillingType> for BillingType {}
