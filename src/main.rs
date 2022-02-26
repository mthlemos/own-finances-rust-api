#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use std::env;
use wither::Result;

mod api_routes;
mod db;
mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<()> {
    // Loading .env variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set on .env");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set on .env");
    // Database connection
    let db = db::start_db(&database_url, &database_name)
        .await
        .expect("Db could not connect");
    println!("Db success!");

    // Launch web server
    if let Err(e) = rocket::build()
        .manage(db) // Passing database ref to all routes
        .mount(
            "/api",
            routes![
                api_routes::get_invoice_query_with_date,
                api_routes::get_invoice_query_without_date,
                api_routes::add_invoice,
                api_routes::edit_invoice,
                api_routes::delete_invoice,
                api_routes::get_categories,
                api_routes::add_categories,
                api_routes::delete_category,
                api_routes::get_billing_types,
                api_routes::add_billing_types,
                api_routes::delete_billing_type,
                index
            ],
        )
        .launch()
        .await
    {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    }

    Ok(())
}
