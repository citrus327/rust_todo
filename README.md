# rust_todo

## Introduction

A simple To-do app that I use to get started with rust lang.

To-dos will be stored in a json file under ProjectDir by default.
MACOS: `/Users/<user_name>/Library/Application Support/RTODO/todo.json`

## Usage

```bash
rtodo add "This is a todo"
rtodo add "This is a todo" --complete
rtodo complete [id]
rtodo uncomplete [id]
rtodo list
rtodo clean
```

## Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] } # data (de)serialize
serde_json = "1.0.108" # json (de)serialize
clap = { version = "4.4.7", features = ["derive", "cargo"] } # Cli argument parser
notify-rust = "4" # XDG notification
prettytable-rs = "^0.10" # print data in table format
directories = "5.0" # get path of sys dir
```

## Knowledege
1. Pattern matching and related error handling.
2. File processing.
3. Path processing.
4. CLI arguments parsing.
5. XDG related notification with extern "C".
6. Crate publish
7. Unit testing
8. System directories
9. Macros

