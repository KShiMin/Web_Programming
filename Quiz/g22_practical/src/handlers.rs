use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use bcrypt::verify;

use crate::models::*;
use crate::auth;

pub async fn register_user(
    pool: &SqlitePool,
    form: RegisterUser,
) -> Result<(), sqlx::Error> {
    // 1. Prepend SALT to the plain password
    let salted = format!("{}{}", SALT, form.password);

    // 2. Bcrypt-hash it (bcrypt will add its own random salt too)
    let hashed = hash(&salted, DEFAULT_COST).unwrap();

    // 3. Insert into your users table
    sqlx::query!(
        r#"
        INSERT INTO users (user_id, username, email, hashed_password, role, team_id)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        Uuid::new_v4(),
        form.username,
        form.email,
        hashed,
        form.role,
        form.team_id,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn login(
    pool: web::Data<SqlitePool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    // 1. Lookup the user by username
    let row = match sqlx::query!(
        "SELECT hashed_password, role FROM users WHERE username = ?",
        req.username
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(r)) => r,
        _ => return HttpResponse::Ok().json({ "status": "failure" }),
    };c

    // 2. Re-apply your fixed salt to the incoming password
    let salted_input = format!("{}{}", SALT, req.password);

    // 3. Verify against the stored bcrypt hash
    if verify(&salted_input, &row.hashed_password).unwrap_or(false) {
        // success → you have row.role if you need it
        HttpResponse::Ok().json({
            "status": "success",
            "role": row.role
        })
    } else {
        HttpResponse::Ok().json({ "status": "failure" })
    }
}
