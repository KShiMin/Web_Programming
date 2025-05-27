use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskError{
    #[error("Invalid task ID provided.")]
    InvalidTaskId,

    #[error("Task not found.")]
    NotFound,

    #[error("Unknown error occurred.")]
    Unknown,
}