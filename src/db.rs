use crate::models::Invoice;
use tokio_compat_02::FutureExt;
use wither::mongodb::Client;
use wither::mongodb::Database;
use wither::{prelude::*, WitherError};

pub async fn start_db(database_url: &str, database_name: &str) -> Result<Database, WitherError> {
    // Connect & sync indexes.
    let db = Client::with_uri_str(database_url)
        .compat()
        .await?
        .database(database_name);
    Invoice::sync(&db).compat().await?;

    let mut inv = Invoice {
        id: None,
        name: "batata".to_string(),
        purchase_date: 2,
        billing_type: "banco".to_string(),
        installments: 15,
        recurring: false,
        category: "batata".to_string(),
    };
    if let Err(e) = inv.save(&db, None).await {
        panic!("Err: {}", e);
    }

    Ok(db)
}
