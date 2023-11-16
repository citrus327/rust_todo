use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, help = "The relative path of json file")]
    pub location: Option<String>,

    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add(AddArgs),

    List,

    Complete(CompleteArgs),

    Clean,
}

#[derive(Args)]
pub struct AddArgs {
    #[arg(name = "TODO_NAME", help = "The name of the To-do")]
    pub name: String,

    #[arg(short, long)]
    pub completed: bool,
}

#[derive(Args)]
pub struct CompleteArgs {
    #[arg(name = "TODO_ID", help = "The id of the To-do")]
    pub id: i32,
}
