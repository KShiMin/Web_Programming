use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Student {
    pub student_id: i64,
    pub name: String,
    pub dob: String,
    pub class_assignment: String,
}

#[derive(Serialize, Deserialize)]
pub struct Staff {
    id: Uuid,
    name: String,
    // password: ,
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    module_code: String,
    name: String,
}

// #[derive(Serialize, Deserialize)]
// pub struct Schedule {
    
// }