use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Enums for bug reports
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Low, 
    Medium, 
    High,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Status {
    Active, 
    Test, 
    Verified, 
    Closed, 
    Reopened,
}

// define different types of roles available in a company
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UserRole {
    Admin,
    Developer
}

#[derive(Serialize, Deserialize)]
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
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)] 
pub struct LoginQuery { 
    pub error: Option<String>,
    pub success: Option<String>, 
} 

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub team_id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: UserRole,
    pub team_id: Uuid,  // link to Team struct to retrieve necessary info.
}

