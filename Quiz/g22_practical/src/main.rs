// Import necessary items from external crates and internal modules to configure and run the web server.
// `actix_web` is used to build web applications and handle HTTP interactions.
// `dotenv` is used to load environment variables from a `.env` file.
// `std::env` is used for accessing environment variables.
// Internal module imports include `handlers` for routing, `models` for data structures, `auth` for authentication, and `db` for database operations.
use actix_web::{web, App, HttpServer};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::Key;
use dotenvy::dotenv;
use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use tera::Tera;
use std::sync::{Arc, Mutex};

mod routes {
    pub mod bugs;
    // pub mod projects;
    // pub mod auth;
    pub mod assign;
}

// Declare internal modules used in this application.
mod handlers; // Handles HTTP request routing and response.
mod models;   // Defines data structures used across the application.
// mod auth;     // Handles authentication logic and utilities.
mod db;       // Contains database initialization and interaction functions.

use crate::handlers::*;



// The `main` function is the application's entry point, running within the `actix_web` runtime.
// It returns a `Result` that can indicate I/O operations' success or failure.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from a `.env` file. This is helpful for configuration management.
    dotenv().ok();

    // Initialize the database connection pool asynchronously and store it in `db_pool`.
    let db_pool = db::init_db().await;
    let tera = Tera::new("templates/**/*").unwrap();
    let secret_key = Key::generate();

    // let project_list: SharedProjectList = Arc::new(Mutex::new(vec![]));

    // Shared, in-memory project list
    let project_list = Arc::new(Mutex::new(Vec::new()));

    // Bundle everything into one state
    let app_state = AppState {
        pool: db_pool.clone(),
        projects: project_list.clone(),
        tera: tera.clone(),
    };

    // Configure and run the HTTP server.
    HttpServer::new(move || {
        App::new()
            // Share the database pool across different parts of the application using application data.
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(false)
                .build())
            // Configure application routes using the `config` function from the `handlers` module.
            .service(
                web::scope("/bugs")
                    // HTML assign form & handler
                    .service(routes::assign::assign_form)
                    .service(routes::assign::post_assign)
                    // JSON endpoints
                    .service(routes::bugs::create_bug)   // POST /bugs/new
                    .service(routes::bugs::list_bugs)    // GET  /bugs
                    // Parameterized, last
                    .service(routes::bugs::get_bug)      // GET    /bugs/{id}
                    .service(routes::bugs::update_bug)   // PATCH  /bugs/{id}
                    .service(routes::bugs::delete_bug)   // DELETE /bugs/{id}
            )
            .route("/login", web::get().to(login_form))
            .route("/login", web::post().to(login_process))
            .route("/api/login", web::post().to(login)) // For API/curl (application/json)
            .route("/home", web::get().to(home))
            .route("/projects", web::get().to(get_projects))
            .route("/projects", web::post().to(create_project))

    })
    // Bind the server to listen on the local machine at port 8080.
    .bind(("127.0.0.1", 8080))?
    .run() // Start the server.
    .await // Await the completion of the server run (this runs indefinitely until shutdown).
}
