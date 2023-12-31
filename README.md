# htodo

[![Rust](https://github.com/citrus327/rust_todo/actions/workflows/rust.yml/badge.svg)](https://github.com/citrus327/rust_todo/actions/workflows/rust.yml)

## Introduction

A simple To-do app that I use to get started with rust lang.

To-dos will be stored in a json file under ProjectDir by default.

macos: `/Users/<user_name>/Library/Application Support/htodo/todo.json`

## Installation

`cargo install htodo`

## Usage

* Initialize

```bash
htodo init
```

* basic
```bash
htodo add "This is a todo"
htodo add "This is a todo" --complete
htodo complete [id]
htodo uncomplete [id]
htodo list
htodo clean
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


## Development

For testing:
`cargo test -- --test-threads 1`


## What's left
1. Search and sorting feature
2. Colored output
3. Add due date to To-Do
4. Notification Icons


## Contribution
This is still a very basic todo app.
Pr is very much welcomed.

