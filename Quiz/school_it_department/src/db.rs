use sqlx::{Pool, Sqlite};
use sqlx::sqlite::SqlitePoolOptions;

pub async fn init_db() -> Pool<Sqlite> {

    SqlitePoolOptions::new()
        .connect("sqlite::memory:") // Connect to an in-memory SQLite database.
        .await // Since database connections are asynchronous operations, await the completion.
        .expect("DB connection failed") // Panic with an error message if the connection fails.
}