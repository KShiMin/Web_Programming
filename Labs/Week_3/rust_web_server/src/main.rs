use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Message{
    content: String,
}

#[get("/greet")]
async fn greet(web::Query(info): web::Query<std::collections::HashMap<String,String>>) -> impl Responder{
    let name = info.get("name").map(String::as_str).unwrap_or("Guest");
    format!("Hello, {}", name)
}

#[get("/json")]
async fn json_response() -> impl Responder{
    let msg = Message{
        content: "This is a JSON response.".to_string(),
    };
    HttpResponse::Ok().json(msg)
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello from Rust Web Server!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
        .service(hello)
        .service(json_response)
        .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
