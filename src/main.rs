#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use rocket::Rocket;
use std::env;
use wither::mongodb::Database;
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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set");
    // Database connection
    let db: Database;
    match db::start_db(&database_url, &database_name).await {
        Ok(dbConn) => {
            println!("Db success!");
            db = dbConn;
        }
        Err(err) => panic!("Db could not connect: {}", err),
    };

    if let Err(e) = rocket::build()
        .manage(db)
        .mount("/api", routes![api_routes::invoice, index])
        .launch()
        .await
    {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    }

    Ok(())
}
