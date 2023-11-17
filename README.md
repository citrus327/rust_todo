# rust_todo

## Introduction
A simple To-do app that I use to get started with rust lang.

To-dos will be stored in a json file under `~/rtodo/todo.json` by default.

## Usage

```bash
rtodo add "This is a todo"
rtodo add "This is a todo" --complete
rtodo remove [id]
rtodo search [name]
rtodo complete [id]
rtodo list
```

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] } # data (de)serialize
serde_json = "1.0.108" # json (de)serialize
clap = { version = "4.4.7", features = ["derive", "cargo"] } # Cli argument parser
notify-rust = "4" # XDG notification
prettytable-rs = "^0.10" # print data in table format
```
