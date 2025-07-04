use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::{Type, FromRow};

/// Store these as UPPERCASE strings in SQLite
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "UPPERCASE",    // matches your CREATE TABLE column values
       type_name = "Severity")]     // optional: the SQL name
pub enum Severity {
    LOW,
    MEDIUM,
    HIGH,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "UPPERCASE", type_name = "Status")]
pub enum Status {
    ACTIVE,
    TEST,
    VERIFIED,
    CLOSED,
    REOPENED,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type)]
#[sqlx(rename_all = "UPPERCASE", type_name = "UserRole")]
pub enum UserRole {
    Admin,
    Developer,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
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
} 

// ASSIGN form
#[derive(Deserialize)]
pub struct AssignForm {
    pub bug_id: Uuid,
    pub developer_id: Uuid,
}


#[derive(Deserialize)]
pub struct NewBug {
    pub title:       String,
    pub description: String,
    pub reported_by: String,
    pub severity:    Option<Severity>,
}

#[derive(Deserialize)]
pub struct BugQuery {
    pub status:   Option<String>,
    pub severity: Option<Severity>,
    pub project:  Option<String>,
}

#[derive(Deserialize)]
pub struct PatchBug {
    pub status:        Option<String>,
    pub severity:      Option<Severity>,
    pub description:   Option<String>,
    pub developer_id:  Option<Uuid>,
    pub project:       Option<String>,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub project_id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub team_id: Uuid,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: UserRole,
    pub team_id: Uuid,  // link to Team struct to retrieve necessary info.
}

