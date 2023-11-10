#![allow(unused)]

mod todos;

use clap::App;

use crate::todos::{Storage, Todo};

// #[derive(Parser)]
// struct Cli {
//     #[arg(help = "The name of the todo")]
//     name: String,

//     #[arg(long, short, help = "Mark todo as completed", name = "finished")]
//     completed: bool,
// }

fn main() {

    // let args = Cli::parse();

    // let storage: Storage = Storage::default();

    // storage.store(Todo {
    //     name: args.name,
    //     completed: args.completed,
    // });
}
