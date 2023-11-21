mod auto_increment_id;
mod cli;
mod dir;
pub mod storage;
use clap::Parser;
use cli::{Cli, Commands};
use dir::{get_todo_file_path, is_todo_file_created, make_todo_file};
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

  match cli.commands {
    Commands::Init => {
      make_todo_file();
      notify("Initialize Successfully")
    }

    Commands::Add(args) => {
      if !is_todo_file_created() {
        notify("Please run `htodo init` first to initialize.");
        return;
      }

      let mut storage: Storage = Storage::new(get_todo_file_path().unwrap().to_str());
      let id = storage.add(args.name.as_str(), args.completed);
      let todo = storage.get_by_id(id);
      notify(format!("{} Added", todo.name).as_str());
    }

    Commands::List => {
      if !is_todo_file_created() {
        notify("Please run `htodo init` first to initialize.");
        return;
      }

      let storage: Storage = Storage::new(get_todo_file_path().unwrap().to_str());
      storage.pretty_print();
    }

    Commands::Complete(args) => {
      if !is_todo_file_created() {
        notify("Please run `htodo init` first to initialize.");
        return;
      }
      let mut storage: Storage = Storage::new(get_todo_file_path().unwrap().to_str());
      storage.complete(args.id);
      let todo = storage.get_by_id(args.id);
      notify(format!("{} Completed", todo.name).as_str());
    }

    Commands::Clean => {
      if !is_todo_file_created() {
        notify("Please run `htodo init` first to initialize.");
        return;
      }
      let mut storage: Storage = Storage::new(get_todo_file_path().unwrap().to_str());

      storage.clean();
      notify("All Todo has been cleaned up");
    }

    Commands::Uncomplete(args) => {
      if !is_todo_file_created() {
        notify("Please run `htodo init` first to initialize.");
        return;
      }
      let mut storage: Storage = Storage::new(get_todo_file_path().unwrap().to_str());

      storage.uncomplete(args.id);
      let todo = storage.get_by_id(args.id);
      notify(format!("{} Uncompleted", todo.name).as_str());
    }
  }
}
