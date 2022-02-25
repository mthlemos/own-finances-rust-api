use crate::models::Invoice;
use futures::stream::StreamExt;
use rocket::serde::json::*;
use rocket::State;
use serde_json::json;
use tokio_compat_02::FutureExt;
use wither::bson::doc;
use wither::mongodb::Database;
use wither::prelude::*;

#[get("/invoices")]
pub async fn invoice(state: &State<Database>) -> Json<Value> {
    let mut invoice_vec: Vec<Invoice> = Vec::new();
    match Invoice::find(&state, None, None).compat().await {
        Ok(mut cursor) => {
            while let Some(Ok(invoice)) = cursor.next().compat().await {
                invoice_vec.push(invoice);
            }
            Json(json!({
                "success": true,
                "result": invoice_vec
            }))
        }
        Err(err) => Json(json!({
            "success": false,
            "result": err.to_string()
        })),
    }
}
