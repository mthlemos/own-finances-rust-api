use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::Model;

#[derive(Debug, Model, Serialize, Deserialize)]
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
