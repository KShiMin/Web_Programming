use actix_web::{web, HttpResponse, Responder};
use sqlx::SqlitePool;
use uuid::Uuid;
use bcrypt::verify;

use crate::models::*;
use crate::auth;


async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    // 1) Look up the mock user
    if let Some(user) = mock_user(&req.username) {
        // 2) Prepend SALT to the provided password
        let salted_input = format!("{}{}", SALT, req.password);

        // 3) Verify against the stored bcrypt hash
        if verify(&salted_input, &user.hashed_password).unwrap_or(false) {
            // Success → return role, or a fake token if you want
            return HttpResponse::Ok().json(json!({
                "status": "success",
                "role": user.role,             // e.g. "Admin" or "Developer"
                "token": "fake-session-token"  // optional
            }));
        }
    }

    // Failure
    HttpResponse::Ok().json(json!({
        "status": "failure"
    }))
}