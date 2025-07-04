// Import necessary items from external crates and internal modules to configure and run the web server.
// `actix_web` is used to build web applications and handle HTTP interactions.
// `dotenv` is used to load environment variables from a `.env` file.
// `std::env` is used for accessing environment variables.
// Internal module imports include `handlers` for routing, `models` for data structures, `auth` for authentication, and `db` for database operations.
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

// Declare internal modules used in this application.
mod handlers; // Handles HTTP request routing and response.
mod models;   // Defines data structures used across the application.
mod auth;     // Handles authentication logic and utilities.
mod db;       // Contains database initialization and interaction functions.

const SALT: &str = "bugtrack2025";

fn mock_user(username: &str) -> Option<NewUser> {
    match username {
        "admin" => {
            let user_id = Uuid = Uuid::new_v4();
            let plain = "adminpass";
            let salted = format!("{}{}", SALT, plain);
            let hashed = hash(&salted, DEFAULT_COST).unwrap();
            let email = "admin@abc.com";
            let team_id: Uuid = Uuid::new_v4();
            Some(NewUser {
                username: username.to_string(),
                password: hashed,
                email: email.to_string(),
                role: UserRole::Admin,      // e.g. "Admin" or "Developer"
                team_id: team_id,
            })
        }
        "dev" => {
            let user_id = Uuid = Uuid::new_v4();
            let plain = "devpass";
            let salted = format!("{}{}", SALT, plain);
            let hashed = hash(&salted, DEFAULT_COST).unwrap();
            let email = "admin@abc.com";
            let team_id: Uuid = Uuid::new_v4();
            Some(NewUser {
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


// The `main` function is the application's entry point, running within the `actix_web` runtime.
// It returns a `Result` that can indicate I/O operations' success or failure.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from a `.env` file. This is helpful for configuration management.
    dotenv().ok();

    // Initialize the database connection pool asynchronously and store it in `db_pool`.
    let db_pool = db::init_db().await;

    // Configure and run the HTTP server.
    HttpServer::new(move || {
        App::new()
            // Share the database pool across different parts of the application using application data.
            .app_data(web::Data::new(db_pool.clone()))
            // Configure application routes using the `config` function from the `handlers` module.
            .route("/login", web::post().to(login))
    })
    // Bind the server to listen on the local machine at port 8080.
    .bind(("127.0.0.1", 8080))?
    .run() // Start the server.
    .await // Await the completion of the server run (this runs indefinitely until shutdown).
}
