use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

// Lab03_Q2
#[derive(Serialize)]
struct StatusField {
    server_name: String,
    version: String,
    uptime: u64
}

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


#[get("/status")]
async fn lab03_q2() -> impl Responder {
    let status = StatusField {
        server_name: "Rust Web Server".to_string(),
        version: "0.1.0".to_string(),
        uptime: 0,
    };

    web::Json(status)
}


#[get("/ping")]
async fn lab03_q1() -> impl Responder{
    HttpResponse::Ok().json("pong")
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
        .service(lab03_q1)
        .service(lab03_q2)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
