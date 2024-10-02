use actix_web::{App, HttpServer};
use sqlx::mysql::MySqlPool;
use crate::config::Config;
use crate::adapters::web::employee_handler::create_employee;
use crate::use_cases::employee_usecases::EmployeeUseCase;
use crate::adapters::db::employee_repo::EmployeeRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration
    let config = Config::from_env();

    // Create a MySQL connection pool
    let pool = MySqlPool::connect(&config.database_url).await.expect("Failed to connect to database");

    // Initialize repository and use case
    let employee_repo = EmployeeRepository::new(pool.clone());
    let employee_use_case = EmployeeUseCase { repo: employee_repo };

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            .data(employee_use_case.clone())
            .service(create_employee)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
