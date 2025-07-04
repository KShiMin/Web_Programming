use serde::{Deserialize, Seralize};
use uuid::Uuid;

// Enums for bug reports
pub enum Severity {
    LOW, 
    MEDIUM, 
    HIGH,
}

pub enum Status {
    ACTIVE, 
    TEST, 
    VERIFIED, 
    CLOSED, 
    REOPENED,
}

// define different types of roles available in a company
pub enum UserRole {
    Admin,
    Developer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bug {
    pub bug_id: Uuid,
    pub title: String,
    pub description: String,
    pub reported_by: String,
    pub severity: Severity,
    pub status: Status,
    pub assigned_to: Uuid,
    pub project: Uuid,
}

// Struct for POST request
#[derive(Deserialize)]
pub struct NewProject{
    pub name: String,
    pub description: String,
}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub team_id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub role: UserRole,
    pub team_id: Uuid,  // link to Team struct to retrieve necessary info.
}

