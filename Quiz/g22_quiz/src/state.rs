use sqlx::SqlitePool;
use tera::Tera;
use std::sync::Mutex;
use crate::models::Project;
use actix_web::cookie::Key;

pub struct AppState {
    pub pool:     SqlitePool,
    pub projects: Mutex<Vec<Project>>,
    pub tera:     Tera,
    pub secret_key: Key, 
}
