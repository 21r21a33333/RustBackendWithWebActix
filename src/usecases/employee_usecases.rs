use crate::domain::employee::Employee;
use crate::adapters::db::employee_repo::EmployeeRepository;

pub struct EmployeeUseCase<R: EmployeeRepository> {
    pub repo: R,
}

impl<R: EmployeeRepository> EmployeeUseCase<R> {
    pub async fn add_employee(&self, name: String, age: u32, department: String) -> Result<(), String> {
        let employee = Employee::new(name, age, department)?;

        // Now that the employee is valid, persist to the database
        self.repo.create_employee(employee).await.map_err(|e| e.to_string())
    }
}
