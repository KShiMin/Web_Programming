// Include different modules created
mod cli;
mod task;
mod error;

// Use different features/libraries from the modules via '::<Library_Name>'
use clap::Parser;
use cli::Cli;
use task::run_task;
use anyhow::Result;

fn main() -> Result<()>{
    let cli = Cli::parse();
    run_task(cli)?;
    Ok(())
}
