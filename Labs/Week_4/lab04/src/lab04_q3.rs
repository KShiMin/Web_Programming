use actix_web:: {web, App, HttpServer, Responder, HttpResponse};
use actix_web:: {dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error};
use futures_util::future::{ready, Ready, LocalBoxFuture};
use std::sync::Mutex;
use std::rc::Rc;
use tokio::time::{sleep, Duration};

pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger 
where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static, 
    B: 'static, 
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleWare<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggerMiddleWare {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggerMiddleWare<S> {
    service: Rc<S>,
}

impl<S,B> Service<ServiceRequest> for LoggerMiddleWare<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error=Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Request: {}", req.path());
        let start_time = std::time::Instant::now(); // Start timer

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            // Log duration
            let duration = start_time.elapsed();
            println!("Response time: {:?}", duration);

            Ok(res)
        })
    }

}

struct AppState{
    app_name: String,
    counter: Mutex<i32>,
    name: String,   // Question 1
}

// Handlers
async fn delayed_response() -> impl Responder{
    sleep(Duration::from_secs(2)).await;
    HttpResponse::Ok().body("Responded after delay.")
}


// Question 2
async fn reset_counter(data: web::Data<AppState>) -> impl Responder{
    let mut count = data.counter.lock().unwrap();
    *count = 0;
    format!("Counter reset to 0!")
}


async fn increment_counter(data: web::Data<AppState>) -> impl Responder{
    let mut count = data.counter.lock().unwrap();
    *count += 1;
    format!("Counter is now: {}", count)
}

async fn read_counter(data: web::Data<AppState>) -> impl Responder{
    let count = data.counter.lock().unwrap();
    // Question 1
    println!("Counter: {}, Name: {}", count, data.name);
    format!("Counter value: {}", count)
}

async fn get_app_name(data: web::Data<AppState>) -> impl Responder{
    format!("App name: {}", data.app_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        app_name: "Rusty Web Server".to_string(),
        counter: Mutex::new(0),
        name: "Shi Min".to_string(),    // Question 1
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger)
            .app_data(state.clone())
            .route("/name", web::get().to(get_app_name))
            .route("/inc", web::post().to(increment_counter))
            .route("/value", web::get().to(read_counter))
            .route("/reset", web::get().to(reset_counter))  // Question 2
            .route("/delay", web::get().to(delayed_response))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}