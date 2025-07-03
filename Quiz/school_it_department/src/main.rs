use actix_web::{web, post, get, App, HttpServer, HttpResponse, Responder};
use dotenvy::dotenv;
use std::env;

mod db;
mod models;
mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_pool = db::init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .configure(handler::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}