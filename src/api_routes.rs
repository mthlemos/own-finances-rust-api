use crate::models::*;
use rocket::serde::json::*;
use rocket::State;
use serde_json::json;
use wither::bson::doc;
use wither::mongodb::Database;

#[get("/invoices")]
pub async fn invoice(state: &State<Database>) -> Json<Value> {
    match Invoice::find_wrapper(&state, None).await {
        Ok(invoices) => {
            // Return json
            Json(json!({
                "success": true,
                "result": invoices
            }))
        }
        Err(err) => {
            // Return json
            Json(json!({
                "success": false,
                "result": err.to_string()
            }))
        }
    }
}
