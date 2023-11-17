#![allow(unused)]

mod auto_increment_id;
mod cli;
pub mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use storage::Storage;

fn main() {
    let cli = Cli::parse();
    let location: String = cli.location.unwrap_or("todo.json".to_string());
    let mut storage: Storage = Storage::new(Some(location.as_str()));

    match cli.commands {
        Commands::Add(args) => {
            storage.add(args.name.as_str(), args.completed);
        }

        Commands::List => {
            storage.pretty_print();
        }

        Commands::Complete(args) => {
            storage.complete(args.id);
        }

        Commands::Clean => {
            storage.clean();
        }

        Commands::Complete(args) => {
            storage.complete(args.id);
        }
    }
}
