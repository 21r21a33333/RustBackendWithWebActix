use crate::domain::employee::Employee;
use sqlx::mysql::MySqlPool;
use std::error::Error;

pub struct EmployeeRepository {
    pool: MySqlPool,
}

impl EmployeeRepository {
    pub fn new(pool: MySqlPool) -> Self {
        EmployeeRepository { pool }
    }

    pub async fn create_employee(&self, employee: Employee) -> Result<(), Box<dyn Error>> {
        sqlx::query!(
            "INSERT INTO employees (name, age, department) VALUES (?, ?, ?)",
            employee.name,
            employee.age,
            employee.department
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_employee(&self, id: i32) -> Result<Employee, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, name, age, department FROM employees WHERE id = ?",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Employee {
            id: Some(row.id),
            name: row.name,
            age: row.age as u32,
            department: row.department,
        })
    }

    // More repository methods like update, delete, etc.
}
