use crate::models::*;
use rocket::serde::json::*;
use rocket::State;
use serde_json::json;
use wither::bson::{doc, oid::ObjectId};
use wither::mongodb::Database;
use wither::Model;

#[get(
    "/invoices?withDate=true&<from_date>&<to_date>&<inv_type>&<category>",
    rank = 1
)]
pub async fn get_invoice_query_with_date(
    state: &State<Database>,
    from_date: Option<i64>,
    to_date: Option<i64>,
    inv_type: Option<String>,
    category: Option<String>,
) -> Json<Value> {
    let mut filter: Option<wither::bson::Document> = None;
    // Check if date was provided
    if from_date == None || to_date == None {
        return Json(json!({
            "success": false,
            "result": "from_date or to_date param missing"
        }));
    } else if inv_type != None && category == None {
        filter = Some(doc! {
            "$and": [
                {
                    "billing_type": inv_type.unwrap()
                },
                {
                    "purchase_date": {
                        "$gte": from_date.unwrap()
                    }
                },
                {
                    "purchase_date": {
                        "$lte": to_date.unwrap()
                    }
                }
            ]
        });
    } else if inv_type == None && category != None {
        filter = Some(doc! {
            "$and": [
                {
                    "category": category.unwrap()
                },
                {
                    "purchase_date": {
                        "$gte": from_date.unwrap()
                    }
                },
                {
                    "purchase_date": {
                        "$lte": to_date.unwrap()
                    }
                }
            ]
        });
    } else if inv_type != None && category != None {
        filter = Some(doc! {
            "$and": [
                {
                    "billing_type": inv_type.unwrap(),
                },
                {
                    "category": category.unwrap()
                },
                {
                    "purchase_date": {
                        "$gte": from_date.unwrap()
                    }
                },
                {
                    "purchase_date": {
                        "$lte": to_date.unwrap()
                    }
                }
            ]
        });
    } else {
        filter = Some(doc! {
            "$and": [
                {
                    "purchase_date": {
                        "$gte": from_date.unwrap()
                    }
                },
                {
                    "purchase_date": {
                        "$lte": to_date.unwrap()
                    }
                }
            ]
        });
    }
    match Invoice::find_wrapper(&state, filter).await {
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

#[get("/invoices?withDate=false&<inv_type>&<category>", rank = 2)]
pub async fn get_invoice_query_without_date(
    state: &State<Database>,
    inv_type: Option<String>,
    category: Option<String>,
) -> Json<Value> {
    let mut filter: Option<wither::bson::Document> = None;
    // Check if date was provided
    if inv_type != None && category == None {
        filter = Some(doc! {
            "billing_type": inv_type.unwrap()
        })
    } else if inv_type == None && category != None {
        filter = Some(doc! {
            "category": category.unwrap()
        })
    } else if inv_type != None && category != None {
        filter = Some(doc! {
            "billing_type": inv_type.unwrap(),
            "category": category.unwrap()
        })
    }
    match Invoice::find_wrapper(&state, filter).await {
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

#[post("/invoices", format = "application/json", data = "<invoice>")]
pub async fn add_invoice(state: &State<Database>, invoice: Json<Invoice>) -> Json<Value> {
    let mut entry: Invoice = invoice.into_inner();
    match entry.save(&state, None).await {
        Ok(()) => {
            // Return json
            Json(json!({
                "success": true,
                "result": "Invoice created!"
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

#[put("/invoices/<id>", format = "application/json", data = "<invoice>")]
pub async fn edit_invoice(
    state: &State<Database>,
    id: String,
    invoice: Json<Invoice>,
) -> Json<Value> {
    let bson_id = ObjectId::with_string(&id).unwrap();

    match Invoice::find_one_and_replace(
        &state,
        doc! {
            "_id": bson_id
        },
        invoice.document_from_instance().unwrap(),
        None,
    )
    .await
    {
        Ok(_) => {
            // Return json
            Json(json!({
                "success": true,
                "result": "Invoice updated!"
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

#[delete("/invoices/<id>")]
pub async fn delete_invoice(state: &State<Database>, id: String) -> Json<Value> {
    let bson_id = ObjectId::with_string(&id).unwrap();

    match Invoice::find_one_and_delete(
        &state,
        doc! {
            "_id": bson_id
        },
        None,
    )
    .await
    {
        Ok(_) => {
            // Return json
            Json(json!({
                "success": true,
                "result": "Invoice deleted!"
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

#[get("/categories")]
pub async fn get_categories(state: &State<Database>) -> Json<Value> {
    match Category::find_wrapper(&state, None).await {
        Ok(categories) => {
            // Return json
            Json(json!({
                "success": true,
                "result": categories
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

#[post("/categories", format = "application/json", data = "<category>")]
pub async fn add_categories(state: &State<Database>, category: Json<Category>) -> Json<Value> {
    let mut entry: Category = category.into_inner();
    match entry.save(&state, None).await {
        Ok(()) => {
            // Return json
            Json(json!({
                "success": true,
                "result": "Category created!"
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

#[delete("/categories/<id>")]
pub async fn delete_category(state: &State<Database>, id: String) -> Json<Value> {
    let bson_id = ObjectId::with_string(&id).unwrap();

    match Category::find_one_and_delete(
        &state,
        doc! {
            "_id": bson_id
        },
        None,
    )
    .await
    {
        Ok(_) => {
            // Return json
            Json(json!({
                "success": true,
                "result": "Category deleted!"
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

#[get("/billing_types")]
pub async fn get_billing_types(state: &State<Database>) -> Json<Value> {
    match BillingType::find_wrapper(&state, None).await {
        Ok(billing_types) => {
            // Return json
            Json(json!({
                "success": true,
                "result": billing_types
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

#[post("/billing_types", format = "application/json", data = "<billing_type>")]
pub async fn add_billing_types(
    state: &State<Database>,
    billing_type: Json<BillingType>,
) -> Json<Value> {
    let mut entry: BillingType = billing_type.into_inner();
    match entry.save(&state, None).await {
        Ok(()) => {
            // Return json
            Json(json!({
                "success": true,
                "result": "Billing Type created!"
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

#[delete("/billing_types/<id>")]
pub async fn delete_billing_type(state: &State<Database>, id: String) -> Json<Value> {
    let bson_id = ObjectId::with_string(&id).unwrap();

    match BillingType::find_one_and_delete(
        &state,
        doc! {
            "_id": bson_id
        },
        None,
    )
    .await
    {
        Ok(_) => {
            // Return json
            Json(json!({
                "success": true,
                "result": "Billing Type deleted!"
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
