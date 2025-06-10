use sqlx::{SqlitePool, FromRow};

#[derive(FromRow, Debug)]
pub struct Task {
	pub id: i64,
	pub name: String,
	pub description: Option<String>,
}

pub async fn get_tasks(pool: &SqlitePool) -> Result<Vec<Task>, sqlx::Error> {
	sqlx::query_as::<_,Task>("SELECT id, name, description FROM tasks")
		.fetch_all(pool)
		.await
}
