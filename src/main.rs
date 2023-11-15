#![allow(unused)]

mod cli;
pub mod storage;
pub mod todos;

use clap::Parser;
use cli::{Cli, Commands};
use storage::Storage;
use todos::Todo;

fn main() {
    let mut storage: Storage = Storage::default(None);

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.commands {
        Commands::Add(args) => {
            &storage.add(Todo::new(args.name.as_str(), args.completed));
        }

        Commands::List => {
            storage.list();
        }
    }
}
