use actix_web::{get, post, web, HttpResponse, Responder};
use actix_session::{Session};
use crate::models::{User, UserRole, LoginQuery, LoginForm};
use crate::state::AppState;
use serde_json::json;
use tera::Context;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;


const SALT: &str = "bugtrack2025";

pub fn mock_user(username: &str) -> Option<User> {
    match username {
        "admin" => {
            let plain = "adminpass";
            let salted = format!("{}{}", SALT, plain);
            let hashed = hash(&salted, DEFAULT_COST).unwrap();
            let email = "admin@abc.com";
            Some(User {
                user_id: Uuid::new_v4(),
                username: username.to_string(),
                password: hashed,
                email: email.to_string(),
                role: UserRole::Admin,      // e.g. "Admin" or "Developer"
                team_id: Uuid::new_v4(),
            })
        }
        "dev" => {
            let plain = "devpass";
            let salted = format!("{}{}", SALT, plain);
            let hashed = hash(&salted, DEFAULT_COST).unwrap();
            let email = "dev@abc.com";
            Some(User {
                user_id: Uuid::new_v4(),
                username: username.to_string(),
                password: hashed,
                email: email.to_string(),
                role: UserRole::Developer,    
                team_id: Uuid::new_v4(),
            })
        }
        _ => None,
    }
}

#[get("/")]
pub async fn login_form( 
    tmpl: web::Data<AppState>, 
    query: web::Query<LoginQuery>, 
) -> impl Responder { 
    let mut ctx = Context::new(); 
    if let Some(err) = &query.error { 
        if err == "1" { 
            ctx.insert("error", "Login Failed! Invalid username or password."); 
        } 
    } 

    let s = tmpl.tera.render("login.html", &ctx).unwrap(); 
    HttpResponse::Ok().body(s) 
}

#[post("/")]
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
            session.insert("role", &user.role)?;
            println!("Status: Success");
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/home"))
                .finish()
            )
        } else {
            println!("Status: Failure");
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/?error=1"))
                .finish()
            )
        };
    }
    
    Ok(HttpResponse::Ok().json(json!({
        "Status": "failure"
    })))
}

#[get("/")]
pub async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

#[get("/home")]
pub async fn home(
    tmpl: web::Data<AppState>,
    session: Session
) -> impl Responder {

    let username: Option<String> = session.get("username").unwrap_or(None);
    let role:     Option<String> = session.get("role").unwrap_or(None);

    match (username, role) {
        (Some(user), Some(role)) => {
            let mut ctx = Context::new();
            ctx.insert("username", &user);
            ctx.insert("role", &role);

            let home = tmpl.tera.render("home.html", &ctx).unwrap();
            HttpResponse::Ok().body(home)
            // HttpResponse::Ok().body(format!("Welcome, {}!", username))
        }
        _ => HttpResponse::Found().append_header(("Location", "/home")).finish(),
    }
}

#[post("/api/login")]
pub async fn login(req: web::Json<LoginForm>, session: Session) -> impl Responder {
    // 1) Look up the mock user
    if let Some(user) = mock_user(&req.username) {
        // 2) Prepend SALT to the provided password
        let salted_input = format!("{}{}", SALT, req.password);

        // 3) Verify against the stored bcrypt hash
        if verify(&salted_input, &user.password).unwrap_or(false) {
            // Store role in session
            session.insert("role", &user.role).unwrap();
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