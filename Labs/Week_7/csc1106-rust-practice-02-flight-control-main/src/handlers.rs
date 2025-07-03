use actix_web::{web, HttpResponse, Responder};
use rusqlite::params;
use std::sync::Mutex;  // Import Mutex here
use rusqlite::Connection;
use crate::models::{Flight, WeatherRequest}; // Ensure WeatherRequest is imported

pub async fn schedule_flight(flight: web::Json<Flight>, data: web::Data<Mutex<rusqlite::Connection>>) -> impl Responder {
    let flight = flight.into_inner();
    
    if flight.flight_plan.is_empty() {
        return HttpResponse::BadRequest().body("Flight plan is required.");
    }

    let current_time = chrono::Utc::now().to_rfc3339();
    if flight.departure_time < current_time {
        return HttpResponse::BadRequest().body("Departure time cannot be in the past.");
    }

    let conn = data.lock().unwrap(); // Lock the connection
    match conn.execute(
        "INSERT INTO flight (flight_id, pilot_id, aircraft_id, flight_plan, departure_time) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![flight.flight_id, flight.pilot_id, flight.aircraft_id, flight.flight_plan, flight.departure_time],
    ) {
        Ok(_) => {
            return HttpResponse::Ok().json(format!(
                "Flight {} scheduled successfully with aircraft {}.",
                flight.flight_id, flight.aircraft_id
            ))
        },
        Err(_) => return HttpResponse::InternalServerError().body("Failed to schedule flight."),
    };
}

pub async fn view_scheduled_flights(data: web::Data<Mutex<Connection>>) -> impl Responder {
    let conn = data.lock().unwrap();
    let mut stmt = conn.prepare("SELECT flight_id, pilot_id, aircraft_id, flight_plan, departure_time FROM flight")
        .unwrap();

    let flight_iter = stmt.query_map([], |row| {
        Ok(Flight {
            flight_id: row.get(0)?,
            pilot_id: row.get(1)?,
            aircraft_id: row.get(2)?,
            flight_plan: row.get(3)?,
            departure_time: row.get(4)?,
        })
    }).unwrap();

    let mut flights = Vec::new();
    for flight in flight_iter {
        flights.push(flight.unwrap());
    }

    HttpResponse::Ok().json(flights)
}
pub async fn retrieve_weather(info: web::Query<WeatherRequest>) -> impl Responder {
    // Logic for weather retrieval
    HttpResponse::Ok().json(format!(
        "Weather data for lat: {}, long: {}",
        info.latitude, info.longitude       // retrieved from web?
    ))
}

pub async fn get_aircraft_details() -> impl Responder {
    // Logic for retrieving aircraft details
    HttpResponse::Ok().body("Aircraft details")
}