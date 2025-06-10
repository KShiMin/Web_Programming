use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, SqlitePool};
use std::env;

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
        .await;

    match result{
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().body(format!("Updated task {}", id)),
        Ok(_) => HttpResponse::NotFound().body(format!("404 Not Found\n")),
        Err(err) => {
            eprintln!("Update error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/tasks/{id}")]
async fn delete_task(
    db_pool: web::Data<SqlitePool>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();

    let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(db_pool.get_ref())
        .await;

    match result{
        Ok(res) if res.rows_affected() > 0 => HttpResponse::Ok().body(format!("Deleted task {}", id)),
        Ok(_) => HttpResponse::NotFound().body(format!("Task {} not found\n", id)),
        Err(err) => {
            eprintln!("Delete error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/tasks")]
async fn create_task(
    db_pool: web::Data<SqlitePool>,
    form: web::Json<CreateTask>,
) -> impl Responder {
    let result = sqlx::query("INSERT INTO tasks (name, description) VALUES (?, ?)")
        .bind(&form.name)
        .bind(&form.description)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(res) => {
            let id = res.last_insert_rowid();
            HttpResponse::Created().json(Task {
                id,
                name: form.name.clone(),
                description: form.description.clone(),
            })
        }
        Err(err) => {
            eprintln!("Insert error: {:?}", err);
            HttpResponse:: InternalServerError().finish()
        }
    }
}

#[get("/tasks")]
async fn get_tasks(db_pool: web::Data<SqlitePool>) -> impl Responder {
    let result = sqlx::query_as::<_,Task>("SELECT id, name, description FROM tasks")
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(tasks) => {
            println!("Fetched {:?} tasks", tasks.len());
            HttpResponse::Ok().json(tasks)
        }
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
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
            .service(create_task)
            .service(delete_task)
            .service(update_task)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
