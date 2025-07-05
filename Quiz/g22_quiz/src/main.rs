mod state;
mod models;
mod email;
mod routes {
    pub mod bugs;
    pub mod projects;
    pub mod auth;
    pub mod assign;
}

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_session::{SessionMiddleware, storage::CookieSessionStore}; 
use actix_web::cookie::Key; 
use dotenvy::dotenv;
use env_logger::init;
use sqlx::SqlitePool;
use tera::Tera;
use std::sync::Mutex;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init();

    // Connect & migrate
    // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL in .env");
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query(include_str!("../migrations/001_create_tables.sql"))
        .execute(&pool).await.unwrap();

    // Tera + in-memory projects
    let tera = Tera::new("templates/**/*").unwrap();
    let projects = Mutex::new(Vec::new());
    let secret_key = Key::generate();
    let state = web::Data::new(AppState { pool, projects, tera, secret_key: secret_key.clone() });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false) // Set to true for production with HTTPS
                .build()
            )
            .app_data(state.clone())

            // **Scope** all `/bugs` routes under one umbrella
            .service(
                web::scope("/bugs")
                    // HTML assign form & handler
                    .service(routes::assign::assign_form)
                    .service(routes::assign::post_assign)

                    // JSON endpoints
                    .service(routes::bugs::create_bug)   // POST /bugs/new
                    .service(routes::bugs::list_bugs)   // for curl if want use
                    .service(routes::bugs::list_bugs_html)    // GET  /bugs

                    // Parameterized, last
                    .service(routes::bugs::view_bug_html)      // GET    /bugs/{id}
                    .service(routes::bugs::get_bug) // for curl if want use
                    .service(routes::bugs::update_bug)   // PATCH  /bugs/{id}
                    .service(routes::bugs::delete_bug)   // DELETE /bugs/{id}
            )

            // Other top-level routes
            .service(routes::projects::get_projects)  // GET  /projects
            .service(routes::projects::create_project)   // POST /projects
            .service(routes::auth::login_process)             // POST /login
            .service(routes::auth::login_form)             // GET /login
            .service(routes::auth::logout) 
            .service(routes::auth::home)             // GET /home
            .service(routes::auth::login)               // POST /api/login for curl
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
