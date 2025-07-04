use actix_web::{web, HttpResponse, Responder};
use actix_session::{Session};
use sqlx::SqlitePool;
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::json;
use tera::{Tera, Context};
use std::sync::{Arc, Mutex};
use crate::AppState;

// mod models;
// mod auth;

use crate::models::*;
// use crate::auth;

const SALT: &str = "bugtrack2025";

fn mock_user(username: &str) -> Option<User> {
    match username {
        "admin" => {
            let user_id: Uuid = Uuid::new_v4();
            let plain = "adminpass";
            let salted = format!("{}{}", SALT, plain);
            let hashed = hash(&salted, DEFAULT_COST).unwrap();
            let email = "admin@abc.com";
            let team_id: Uuid = Uuid::new_v4();
            Some(User {
                user_id: user_id,
                username: username.to_string(),
                password: hashed,
                email: email.to_string(),
                role: UserRole::Admin,      // e.g. "Admin" or "Developer"
                team_id: team_id,
            })
        }
        "dev" => {
            let user_id: Uuid = Uuid::new_v4();
            let plain = "devpass";
            let salted = format!("{}{}", SALT, plain);
            let hashed = hash(&salted, DEFAULT_COST).unwrap();
            let email = "dev@abc.com";
            let team_id: Uuid = Uuid::new_v4();
            Some(User {
                user_id: user_id,
                username: username.to_string(),
                password: hashed,
                email: email.to_string(),
                role: UserRole::Developer,    
                team_id: team_id,
            })
        }
        _ => None,
    }
}


// Error Display in html
pub async fn login_form( 
    tmpl: web::Data<Tera>, 
    query: web::Query<LoginQuery>, 
) -> impl Responder { 
    let mut ctx = tera::Context::new(); 
    if let Some(err) = &query.error { 
        if err == "1" { 
            ctx.insert("error", "Login Failed! Invalid username or password."); 
        } 
    } 

    let s = tmpl.render("login.html", &ctx).unwrap(); 
    HttpResponse::Ok().body(s) 
}

// Login Process 
pub async fn login_process(
    form: web::Form<LoginForm>,
    session: Session,
) -> actix_web::Result<HttpResponse> {
    if let Some(user) = mock_user(&form.username) {
        // 2) Prepend SALT to the provided password
        let salted_input = format!("{}{}", SALT, form.password);

        // 3) Verify against the stored bcrypt hash
        if verify(&salted_input, &user.password).unwrap_or(false) {
            session.insert("username", &form.username)?;
            println!("Status: Success");
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/home"))
                .finish()
            )
        } else {
            println!("Status: Failure");
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/login?error=1"))
                .finish()
            )
        };
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "Status": "failure"
    })))
}

// Home Page Logic
pub async fn home(session: Session) -> impl Responder {
    match session.get::<String>("username") {
        Ok(Some(username)) => HttpResponse::Ok().body(format!("Welcome, {}!", username)),
        _ => HttpResponse::Found().append_header(("Location", "/login")).finish(),
    }
}

pub async fn login(req: web::Json<LoginForm>) -> impl Responder {
    // 1) Look up the mock user
    if let Some(user) = mock_user(&req.username) {
        // 2) Prepend SALT to the provided password
        let salted_input = format!("{}{}", SALT, req.password);

        // 3) Verify against the stored bcrypt hash
        if verify(&salted_input, &user.password).unwrap_or(false) {
            // Success → return role, or a fake token if you want
            return HttpResponse::Ok().json(json!({
                "status": "success",
                "role": user.role,             // e.g. "Admin" or "Developer"
                "token": "fake-session-token"  // optional
            }));
        }
    }

    // Failure
    HttpResponse::Ok().json(json!({
        "status": "failure"
    }))
}

// Question 2 - CURL only
pub type SharedProjectList = Arc<Mutex<Vec<Project>>>;

pub async fn create_project(
    session: Session,
    data: web::Data<SharedProjectList>,
    new_proj: web::Json<NewProject>,
) -> impl Responder {
    // Extract role from session
    let role = session.get::<String>("role").unwrap_or(None);

    if role.as_deref() != Some("Admin") {
        return HttpResponse::Unauthorized().json(json!({
            "Error": "Only admin users can create projects."
        }));
    }

    let mut projects = data.lock().unwrap();
    let project = Project {
        project_id: Uuid::new_v4(),
        name: new_proj.name.clone(),
        description: new_proj.description.clone(),
    };
    projects.push(project);

    HttpResponse::Ok().json(json!({
        "Status": "Project added successfully"
    }))
}

pub async fn get_projects(data: web::Data<SharedProjectList>) -> impl Responder {
    let projects = data.lock().unwrap();
    HttpResponse::Ok().json(&*projects)
}

