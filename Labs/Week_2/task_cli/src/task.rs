use crate::cli::{Cli, Commands};
use crate::error::TaskError;
use anyhow::Result;

pub fn run_task(cli:Cli) -> Result<()>{
    match cli.command{
        Commands::Add { name } => {
            println!("Task added: {}", name);
        }
        Commands::Remove { id } => {
            if id == 0{
                return Err(TaskError::InvalidTaskId.into());
            };
            println!("Task removed with id {}", id);                
        }
        Commands::List => {
            println!("Listing tasks (mock): Task 1, Task 2");
        }
    }

    Ok(())
}