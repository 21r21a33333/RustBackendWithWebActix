use actix_web::{post, web, HttpResponse, Responder};
use crate::use_cases::employee_usecases::EmployeeUseCase;

#[post("/employees")]
pub async fn create_employee(
    data: web::Json<CreateEmployeeData>,
    use_case: web::Data<EmployeeUseCase<impl EmployeeRepository>>,
) -> impl Responder {
    let result = use_case
        .add_employee(data.name.clone(), data.age, data.department.clone())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Employee created successfully"),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateEmployeeData {
    name: String,
    age: u32,
    department: String,
}
