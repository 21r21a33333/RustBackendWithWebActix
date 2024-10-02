use validator::Validate;

#[derive(Debug, Validate)]
pub struct Employee {
    pub id: Option<i32>,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(range(min = 18, max = 65, message = "Age must be between 18 and 65"))]
    pub age: u32,

    #[validate(length(min = 1, message = "Department cannot be empty"))]
    pub department: String,
}

impl Employee {
    pub fn new(name: String, age: u32, department: String) -> Result<Self, String> {
        let employee = Employee {
            id: None,
            name,
            age,
            department,
        };

        // Validate employee entity
        if let Err(validation_errors) = employee.validate() {
            return Err(validation_errors.to_string());
        }

        Ok(employee)
    }
}
