// src/routes/assign.rs

use actix_web::{get, post, web, HttpResponse, Responder};
use tera::Context;
use crate::AppState;
use crate::models::AssignForm;
use sqlx::query;
use uuid::Uuid;
// use crate::email::send_email;

// GET /bugs/assign → renders the form
#[get("/assign")]
pub async fn assign_form(state: web::Data<AppState>) -> impl Responder {
    let devs = vec![1, 2, 3];
    let rows = query!("SELECT bug_id, title FROM bugs")
        .fetch_all(&state.pool).await.unwrap();

    let bugs: Vec<(i64, String)> = rows
        .into_iter()
        .map(|r| (r.bug_id, r.title))
        .collect();

    let mut ctx = Context::new();
    ctx.insert("bugs", &bugs);
    ctx.insert("devs",  &devs);

    let html = state.tera.render("assign_form.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

// POST /bugs/assign → processes the form
#[post("/assign")]
pub async fn post_assign(
    state: web::Data<AppState>,
    form: web::Form<AssignForm>,
) -> impl Responder {
    // let valid_devs: Vec<Uuid>
    // let valid_devs = vec![1, 2, 3];
    let rows = sqlx::query!(
        // we tell SQLx “the `user_id` column maps to a `Uuid`”
        r#"
        SELECT user_id as "user_id: Uuid"
        FROM users
        WHERE role = ?
        "#,
        UserRole::Developer as _
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("DB error: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    // now collect into a Vec<Uuid>
    let valid_devs: Vec<Uuid> = rows.into_iter()
        .map(|r| r.user_id)
        .collect();

    let mut ctx = Context::new();

    // 1) Validate developer_id → 400
    if !valid_devs.contains(&form.developer_id) {
        ctx.insert("message", "Error: invalid developer selected");
        let html = state.tera.render("confirm.html", &ctx).unwrap();
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body(html);
    }

    // 2) Attempt the UPDATE
    let res = query!(
        "UPDATE bugs SET developer_id = ? WHERE bug_id = ?",
        form.developer_id,
        form.bug_id
    )
    .execute(&state.pool).await.unwrap();

    // 3a) If no rows affected → invalid bug → 404
    if res.rows_affected() == 0 {
        ctx.insert("message", "Error: invalid bug ID");
        let html = state.tera.render("confirm.html", &ctx).unwrap();
        return HttpResponse::NotFound()
            .content_type("text/html")
            .body(html);
    }

    if res.rows_affected() == 1 {
    ctx.insert("message", &format!("Assigned bug {} → dev {}", form.bug_id, form.developer_id));

    // Send notification to admin (or to the developer)
    let admin = std::env::var("ADMIN_EMAIL").unwrap();
    let subject = format!("Bug #{} assigned", form.bug_id);
    let body = format!(
        "Bug #{} has been assigned to developer {}",
        form.bug_id, form.developer_id
    );
    // actix_web::rt::spawn(async move {
    //     let _ = send_email(&admin, &subject, &body).await;
    // });
} else {
    ctx.insert("message", "Error: invalid bug ID");
}

    // 3b) Success → 200
    ctx.insert("message", &format!(
        "Assigned bug {} → dev {}",
        form.bug_id, form.developer_id
    ));
    let html = state.tera.render("confirm.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}
