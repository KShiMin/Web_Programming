// CLI command set ups
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task-cli")]
#[command(about = "A simple task CLI manager", long_about = None)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands{
    Add { name:String },
    Remove { id: u32 },
    List,
}