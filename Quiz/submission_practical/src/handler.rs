use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
// use std::error:: AppError;
// use error::AppError;

use crate::models::*;
// use crate::error::*;
use crate::auth;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/students/enroll").route(web::post().to(enrollment)))
        .service(web::resource("/login").route(web::post().to(login)));
}

async fn login(pool: web::Data<SqlitePool>, body: web::Json<User>) -> impl Responder {
    // Simulate login logic. Typically you would verify user credentials against the database.
    if body.username == "staff1" {
        // If the username matches "admin", a new token is created using your auth logic.
        let token = auth::create_token(Uuid::new_v4());
        // Respond with a 200 OK status and include the token as JSON.
        HttpResponse::Ok().json(serde_json::json!({ "token": token }))
    } else {
        // If authentication fails, respond with a 401 Unauthorized status.
        HttpResponse::Unauthorized().finish()
    }
}


async fn enrollment(_pool: web::Data<SqlitePool>, form: web::Json<Student>) -> impl Responder{
    let res = sqlx::query("INSERT INTO student (name, dob, class_assignment) VALUES (? ? ?)")
        .bind(&form.name)
        .bind(&form.dob)
        .bind(&form.class_assignment)
        .execute(_pool.get_ref())
        .await;
        // .map_err(AppError::Database)?;

    let student_id = res.last_insert_rowid();


    match res{
        Ok(res) => {
            HttpResponse::Created().json(Student {
                student_id,
                name: form.name.clone(),
                dob: form.dob.clone(),
                class_assignment: form.class_assignment.clone(),
        })
        }
        Err(err) => {
            eprintln!("Insert error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
    
}