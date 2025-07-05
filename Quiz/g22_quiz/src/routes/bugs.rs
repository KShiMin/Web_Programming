use actix_web::{post, get, patch, delete, web, HttpResponse, Responder};
use sqlx::Row;
use crate::models::{Bug, NewBug, BugQuery, PatchBug};
use crate::state::AppState;
use crate::email::send_email;
use crate::routes::auth::mock_user;
use tera::Context;

// 1) Create new bug: POST /bugs/new
#[post("/new")]
pub async fn create_bug(
    state: web::Data<AppState>,
    b: web::Json<NewBug>,
) -> impl Responder {
    let mut tx = state.pool.begin().await.unwrap();
    sqlx::query(
        "INSERT INTO bugs (title, description, reported_by, severity) VALUES (?, ?, ?, ?)"
    )
    .bind(&b.title)
    .bind(&b.description)
    .bind(&b.reported_by)
    .bind(&b.severity)
    .execute(&mut tx).await.unwrap();

    let row = sqlx::query("SELECT last_insert_rowid()")
        .fetch_one(&mut tx).await.unwrap();
    let bug_id: i64 = row.get::<i64, _>(0);
    tx.commit().await.unwrap();

    // Fetch the full bug record
    let bug = sqlx::query_as::<_, Bug>("SELECT * FROM bugs WHERE bug_id = ?")
        .bind(bug_id)
        .fetch_one(&state.pool).await.unwrap();

    // Spawn the email task *before* returning
    let subject = format!("New bug #{} reported", bug.bug_id);
    let body = format!(
        "Title: {}\nReporter: {}\nSeverity: {}\n\n{}",
        bug.title, bug.reported_by, bug.severity, bug.description
    );

    if let Some(admin_user) = mock_user("admin") {
        let admin_email = admin_user.email.clone();
        actix_web::rt::spawn(async move {
            let _ = send_email(&admin_email, &subject, &body).await;
        });
    } else {
        // fallback or just skip sending
        println!("No admin user found in mock store, skipping email");
    }

    // Now return the response
    HttpResponse::Created().json(bug)
}


// 2) List all bugs: GET /bugs
#[get("")]
pub async fn list_bugs(
    state: web::Data<AppState>,
    qry: web::Query<BugQuery>,
) -> impl Responder {
    let mut sql = "SELECT * FROM bugs".to_string();
    let mut clauses = Vec::new();
    if qry.status.is_some()   { clauses.push("status = ?"); }
    if qry.severity.is_some() { clauses.push("severity = ?"); }
    if qry.project.is_some()  { clauses.push("project = ?"); }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }

    let mut q = sqlx::query_as::<_, Bug>(&sql);
    if let Some(s) = &qry.status    { q = q.bind(s); }
    if let Some(s) = &qry.severity  { q = q.bind(s); }
    if let Some(p) = &qry.project   { q = q.bind(p); }

    let bugs = q.fetch_all(&state.pool).await.unwrap();
    HttpResponse::Ok().json(bugs)
}

#[get("/view")]
pub async fn list_bugs_html(
    state: web::Data<AppState>
) -> impl Responder {
    let bugs = sqlx::query_as::<_, Bug>("SELECT * FROM bugs")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    let mut ctx = Context::new();
    ctx.insert("bugs", &bugs);

    let body = state.tera.render("bugs_list.html", &ctx).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

// 3) Get a bug by id: GET /bugs/{id}
#[get("/{id}")]
pub async fn get_bug(
    state: web::Data<AppState>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Bug>("SELECT * FROM bugs WHERE bug_id = ?")
        .bind(id).fetch_optional(&state.pool).await.unwrap()
    {
        Some(b) => HttpResponse::Ok().json(b),
        None    => HttpResponse::NotFound().body("Bug not found"),
    }
}

// View one bug’s details
#[get("/view/{id}")]
pub async fn view_bug_html(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match sqlx::query_as::<_, Bug>(
            "SELECT * FROM bugs WHERE bug_id = ?"
        )
        .bind(id.clone())
        .fetch_optional(&state.pool)
        .await
        .unwrap()
    {
        Some(bug) => {
            let mut ctx = Context::new();
            ctx.insert("bug", &bug);
            let body = state.tera.render("bug_detail.html", &ctx).unwrap();
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(body)
        }
        None => HttpResponse::NotFound().body("Bug not found"),
    }
}

// 4) Patch a bug: PATCH /bugs/{id}
#[patch("/{id}")]
pub async fn update_bug(
    state: web::Data<AppState>,
    path: web::Path<i64>,
    p: web::Json<PatchBug>,
) -> impl Responder {
    let id = path.into_inner();
    let mut sets = Vec::new();
    if p.status.is_some()       { sets.push("status = ?"); }
    if p.severity.is_some()     { sets.push("severity = ?"); }
    if p.description.is_some()  { sets.push("description = ?"); }
    if p.developer_id.is_some() { sets.push("developer_id = ?"); }
    if p.project.is_some()      { sets.push("project = ?"); }

    if sets.is_empty() {
        return HttpResponse::BadRequest().body("No fields to update");
    }

    let sql = format!("UPDATE bugs SET {} WHERE bug_id = ?", sets.join(", "));
    let mut q = sqlx::query(&sql);
    if let Some(s) = &p.status       { q = q.bind(s); }
    if let Some(s) = &p.severity     { q = q.bind(s); }
    if let Some(d) = &p.description  { q = q.bind(d); }
    if let Some(dev) = p.developer_id { q = q.bind(dev); }
    if let Some(pr) = &p.project     { q = q.bind(pr); }
    let res = q.bind(id).execute(&state.pool).await.unwrap();

    if res.rows_affected() == 1 {
        let updated = sqlx::query_as::<_, Bug>("SELECT * FROM bugs WHERE bug_id = ?")
            .bind(id)
            .fetch_one(&state.pool).await.unwrap();
        HttpResponse::Ok().json(updated)
    } else {
        HttpResponse::NotFound().body("Bug not found")
    }
}

// 5) Delete a bug: DELETE /bugs/{id}
#[delete("/{id}")]
pub async fn delete_bug(
    state: web::Data<AppState>,
    path: web::Path<i64>,
) -> impl Responder {
    let id = path.into_inner();
    let res = sqlx::query("DELETE FROM bugs WHERE bug_id = ?")
        .bind(id)
        .execute(&state.pool).await.unwrap();
    if res.rows_affected() == 1 {
        HttpResponse::Ok().body(format!("Deleted bug {}", id))
    } else {
        HttpResponse::NotFound().body("Bug not found")
    }
}
