// Import necessary items from the `sqlx` crate for SQLite database connection pooling.
// `Pool` is used to manage a pool of database connections, while `Sqlite` and `SqlitePoolOptions` are specific to SQLite.
use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;

// Define an asynchronous function `init_db` that initializes a connection pool to an SQLite database.
// This function returns a `Pool<Sqlite>` type, which represents a pool of SQLite connections.
// pub async fn init_db() -> Pool<Sqlite> {
//     // Create a new instance of `SqlitePoolOptions` to configure the connection pool settings.
//     SqlitePoolOptions::new()
//         .connect("sqlite::memory:") // Connect to an in-memory SQLite database.
//         .await // Since database connections are asynchronous operations, await the completion.
//         .expect("DB connection failed") // Panic with an error message if the connection fails.
// }

pub async fn init_db() -> Pool<Sqlite> {
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("DB connection failed");


    
    // run migrations / DDL here:
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS teams (
            team_id   TEXT PRIMARY KEY,
            name      TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS projects (
            project_id   TEXT PRIMARY KEY,
            name         TEXT NOT NULL,
            description  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS users (
            user_id       TEXT PRIMARY KEY,
            username      TEXT NOT NULL UNIQUE,
            password      TEXT NOT NULL,
            email         TEXT NOT NULL,
            role          TEXT NOT NULL,     
            team_id       TEXT NOT NULL,
            FOREIGN KEY(team_id) REFERENCES teams(team_id)
        );

        CREATE TABLE IF NOT EXISTS bugs (
            bug_id        TEXT PRIMARY KEY,
            title         TEXT NOT NULL,
            description   TEXT NOT NULL,
            reported_by   TEXT NOT NULL,
            severity      TEXT NOT NULL,
            status        TEXT NOT NULL,
            assigned_to   TEXT NOT NULL,
            project       TEXT NOT NULL,
            FOREIGN KEY(assigned_to) REFERENCES users(user_id),
            FOREIGN KEY(project)     REFERENCES projects(project_id)
        );
        "#,
    )
    .execute(&pool)
    .await?;

    pool
}

