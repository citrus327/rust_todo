#![allow(unused)]

mod auto_increment_id;
mod cli;
pub mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use notify_rust::Notification;
use storage::Storage;

fn notify(str: &str) {
    let _ = Notification::new()
        .summary(format!("{} Notification", env!("CARGO_PKG_NAME")).as_str())
        .body(str)
        .show();
}

fn main() {
    let cli = Cli::parse();
    let location: String = cli.location.unwrap_or("todo.json".to_string());
    let mut storage: Storage = Storage::new(Some(location.as_str()));

    match cli.commands {
        Commands::Add(args) => {
            let id = storage.add(args.name.as_str(), args.completed);
            let todo = storage.get_by_id(id);
            notify(format!("{} Added", todo.name).as_str());
        }

        Commands::List => {
            storage.pretty_print();
        }

        Commands::Complete(args) => {
            storage.complete(args.id);
            let todo = storage.get_by_id(args.id);
            notify(format!("{} Completed", todo.name).as_str());
        }

        Commands::Clean => {
            storage.clean();
            notify("All Todo has been cleaned up");
        }

        Commands::Uncomplete(args) => {
            storage.uncomplete(args.id);
            let todo = storage.get_by_id(args.id);
            notify(format!("{} Uncompleted", todo.name).as_str());
        }
    }
}
