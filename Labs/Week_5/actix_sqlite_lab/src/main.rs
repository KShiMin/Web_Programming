mod error;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, SqlitePool};
use std::env;
use error::AppError;

#[derive(Deserialize)]
struct CreateTask {
    name: String,
    description: Option<String>,
}

#[derive(Serialize, FromRow, Debug)]
struct Task {
    id: i64,
    name: String,
    description: Option<String>,
}

// Question 2
#[put("/tasks/{id}")]
async fn update_task(
    db_pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
    form: web::Json<CreateTask>,
) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query("UPDATE tasks SET name = ? WHERE id = ?")
        .bind(&form.name)
        .bind(id)
        .execute(db_pool.get_ref())
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound(format!("404 Not Found\n")))
    } else {
        Ok(HttpResponse::Ok().body(format!("Updated task {}", id)))
    }
}

#[delete("/tasks/{id}")]
async fn delete_task(
    db_pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppError>{
    let id = path.into_inner();

    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(db_pool.get_ref())
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        Err(AppError::NotFound(format!("Task {} not found", id)))
    } else {
        Ok(HttpResponse::Ok().body(format!("Deleted task {}", id)))
    }

}

#[post("/tasks")]
async fn create_task(
    db_pool: web::Data<SqlitePool>,
    form: web::Json<CreateTask>
) -> Result<impl Responder, AppError>{
    let result = sqlx::query("INSERT INTO tasks (name, description) VALUES (?, ?)")
        .bind(&form.name)
        .bind(&form.description)
        .execute(db_pool.get_ref())
        .await
        .map_err(AppError::Database)?;

    let id = result.last_insert_rowid();

    Ok(HttpResponse::Created().json(Task {
        id,
        name: form.name.clone(),
        description: form.description.clone()
    }))
}


#[get("/tasks/{id}")]
async fn get_task_by_id(
	db_pool: web::Data<SqlitePool>,
	path: web::Path<i64>,
) -> Result<impl Responder, AppError> {
	let id = path.into_inner();
	let task = sqlx::query_as::<_, Task>("SELECT id, name, description FROM tasks WHERE id = ?")
        .bind(id)
        .fetch_optional(db_pool.get_ref())
        .await
        .map_err(AppError::Database)?;

	match task {
		Some(t) => Ok(HttpResponse::Ok().json(t)),
		None => Err(AppError::NotFound(format!("Task with id {} not found", id))),
	}
}


#[get("/tasks")]
async fn get_tasks(db_pool: web::Data<SqlitePool>) -> Result<impl Responder, AppError> {
    let tasks = sqlx::query_as::<_, Task>("SELECT id, name, description FROM tasks")
        .fetch_all(db_pool.get_ref())
        .await
        .map_err(AppError::Database)?;
    
    // Question 3
    println!("Fetched {:?} tasks", tasks.len());
    Ok(HttpResponse::Ok().json(tasks))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let db_pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to SQLite");

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(get_tasks)
	        .service(get_task_by_id)
            .service(create_task)
            .service(delete_task)
            .service(update_task)	// Question 2
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}