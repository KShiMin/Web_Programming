use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// BUG model
#[derive(Serialize, FromRow)]
pub struct Bug {
    pub bug_id:       i64,
    pub title:        String,
    pub description:  String,
    pub reported_by:  String,
    pub severity:     String,
    pub status:       String,
    pub project:      Option<String>,
    pub developer_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct NewBug {
    pub title:       String,
    pub description: String,
    pub reported_by: String,
    pub severity:    String,
}

#[derive(Deserialize)]
pub struct BugQuery {
    pub status:   Option<String>,
    pub severity: Option<String>,
    pub project:  Option<String>,
}

#[derive(Deserialize)]
pub struct PatchBug {
    pub status:        Option<String>,
    pub severity:      Option<String>,
    pub description:   Option<String>,
    pub developer_id:  Option<i64>,
    pub project:       Option<String>,
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

// PROJECT models
#[derive(Serialize, Deserialize)]
pub struct Project {
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
}

// Struct for POST request
#[derive(Deserialize)]
pub struct NewProject{
    pub name: String,
    pub description: String,
}

// ASSIGN form
#[derive(Deserialize)]
pub struct AssignForm {
    pub bug_id:      i64,
    pub developer_id: i64,
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

// define different types of roles available in a company
#[derive(serde::Serialize, serde::Deserialize)]
pub enum UserRole {
    Admin,
    Developer
}