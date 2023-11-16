#![allow(unused)]

mod cli;
pub mod storage;
pub mod todos;

use clap::Parser;
use cli::{Cli, Commands};
use storage::Storage;
use todos::Todo;

fn main() {
    let cli = Cli::parse();
    let location: String = cli.location.unwrap_or("todo.json".to_string());
    let mut storage: Storage = Storage::new(Some(location.as_str()));

    match cli.commands {
        Commands::Add(args) => {
            storage.add(Todo::new(args.name.as_str(), args.completed));
        }

        Commands::List => {
            storage.pretty_print();
        }
    }
}
