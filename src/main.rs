#![allow(warnings)]
use actix_web::{
    dev::Path,
    get,
    http::StatusCode,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;
use tokio::time::{interval, sleep, Duration};
use chrono::Utc;

mod config;
use config::database_connection;

mod adaptors;
use adaptors::webhandlers::depth_handler::*;
use adaptors::webhandlers::earnings_handler::*;
use adaptors::webhandlers::runepool_handler::*;
use adaptors::webhandlers::swaps_handler::*;

mod utilities;
use utilities::Fetch_data::{fetch_depth_main, fetch_swaps_main, fetch_runepool_main, fetch_earnings_main};

#[get("/")]
async fn index() -> impl Responder {
    "server index route hit"
}

async fn run_cron_job() {
    let mut interval = interval(Duration::from_secs(3600)); // 1 hour (3600 seconds)
    loop {
        interval.tick().await; // Wait for the next tick
        let start_time = Utc::now();
        println!("Running scheduled data fetch at {:?}", start_time);

        // Fetch depth data
        if let Err(e) = fetch_depth_main().await {
            eprintln!("Failed to fetch depth data: {:?}", e);
        }

        // Fetch swaps data
        if let Err(e) = fetch_swaps_main().await {
            eprintln!("Failed to fetch swaps data: {:?}", e);
        }

        // Fetch rune pool data
        if let Err(e) = fetch_runepool_main().await {
            eprintln!("Failed to fetch rune pool data: {:?}", e);
        }

        // Fetch earnings data
        if let Err(e) = fetch_earnings_main().await    {
            eprintln!("Failed to fetch earnings data: {:?}", e);
        }

        let end_time = Utc::now();
        println!("Data fetch completed at {:?}, duration: {:?}", end_time, end_time - start_time);
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    // Establish the database connection
    let database = database_connection()
        .await
        .expect("Failed to create dbpool");
    println!("Connected to database");

        actix_web::rt::spawn(run_cron_job());
    
    // Start the Actix web server
    let server=HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(index)
            .service(get_runepool_history)
            .service(get_depth_and_history)
            .service(get_swaps_history)
            .service(get_earnings)
    })
    .bind(("127.0.0.1", 3000))?
    .run();
    println!("Server running at http://localhost:3000");
    server.await

}
