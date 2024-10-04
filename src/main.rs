use actix_web::{
    dev::Path,
    get,
    http::StatusCode,
    web::{self, Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::Serialize;

mod config;
use config::database_connection;

mod adaptors;
use adaptors::webhandlers::runepool_handler::get_runepool_history;

#[get("/")]
async fn index() -> impl Responder {
    "server index route hit"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = database_connection()
        .await
        .expect("Failed to create dbpool");
    println!("Connected to database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database.clone()))
            .service(index)
            .service(get_runepool_history)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
