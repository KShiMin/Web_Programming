use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::{rc::Rc, task::{Context, Poll}};

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


#[get("/protected")]
async fn protected_route() -> impl Responder {
    HttpResponse::Ok().body("You have accessed a protected route!")
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

// MiddleWare
pub struct AuthMiddleware;


impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<actix_web::body::EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let valid = matches!(
            auth_header.and_then(|h| h.to_str().ok()),
            Some("Bearer secret123")
        );

        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            if !valid {
                let response = HttpResponse::Unauthorized().finish();
                println!("{}", response.status());
                return Ok(req.into_response(response.map_into_right_body()));
            }

            srv.call(req).await.map(|res| res.map_into_left_body())
        })
    }
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
        .service(
            web::scope("")
                .wrap(AuthMiddleware)
                .service(protected_route)
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
