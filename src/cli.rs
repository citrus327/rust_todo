use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddArgs),

    List,
}

#[derive(Args)]
pub struct AddArgs {
    #[arg(name = "TODO_NAME", help = "The name of the To-do")]
    pub name: String,

    #[arg(short, long)]
    pub completed: bool,
}
