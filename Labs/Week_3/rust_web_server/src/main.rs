use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse{
    status: String,
    token: Option<String>,
}

#[derive(Serialize)]
struct Message{
    content: String,
}

#[post("/login")]
async fn login(req: web::Json<LoginRequest>) -> impl Responder{
    if req.username == "admin" && req.password == "password"{
        let res = LoginResponse {
            status: "Sucess".to_string(),
            token: Some("fake-jwt-token".to_string()),
        };
        HttpResponse::Ok().json(res)
    } else {
        let res = LoginResponse{
            status: "failure".to_string(),
            token: None,
        };
        HttpResponse::Unauthorized().json(res)
    }
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
        .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
