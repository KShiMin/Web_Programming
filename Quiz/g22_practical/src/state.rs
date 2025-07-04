use sqlx::SqlitePool;
use tera::Tera;
use std::sync::{Arc, Mutex};
use crate::models::Project;

/// Cloneable state shared across Actix workers
#[derive(Clone)]
pub struct AppState {
    /// SQLite connection pool
    pub pool: SqlitePool,
    /// In-memory list of projects (protected by a Mutex)
    pub projects: Arc<Mutex<Vec<Project>>>,
    /// Tera template engine
    pub tera: Tera,
}