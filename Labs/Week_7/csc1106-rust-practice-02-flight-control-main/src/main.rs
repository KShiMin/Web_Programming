use actix_web::{web, App, HttpServer};
use actix_files as fs;
use rusqlite::Connection;
use std::sync::Mutex;

mod handlers;
mod models;
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute(
        "CREATE TABLE flight (
                  flight_id     INTEGER PRIMARY KEY,
                  pilot_id      INTEGER NOT NULL,
                  aircraft_id   INTEGER NOT NULL,
                  flight_plan   TEXT NOT NULL,
                  departure_time TEXT NOT NULL
                  )",
        [],
    ).unwrap();

    // Wrap the connection in a Mutex
    let data = web::Data::new(Mutex::new(conn));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(web::scope("/flights")
                .route("/schedule", web::post().to(schedule_flight))
                .route("/view", web::get().to(view_scheduled_flights))) // New route
            .route("/weather", web::get().to(retrieve_weather))
            .route("/aircraft/details", web::get().to(get_aircraft_details))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}