use crate::todos::Todo;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs};

pub struct Storage {
    pub todos: Vec<Todo>,
    pub location: PathBuf,
}

fn read_todos_from_file(location: &PathBuf) -> Result<Vec<Todo>, serde_json::Error> {
    if let Ok(content) = fs::read_to_string(&location) {
        let result = serde_json::from_str(content.as_str()).unwrap_or_else(|err| {
            println!("Failed to read content, {}", err);
            vec![]
        });
        Ok(result)
    } else {
        Ok(vec![])
    }
}

impl Storage {
    fn default(file_path: Option<&str>) -> Self {
        let cwd = env::current_dir().unwrap();
        let location = if let Some(path) = file_path {
            cwd.join(path)
        } else {
            cwd.join("todo.json")
        };

        let results = read_todos_from_file(&location).unwrap();

        Self {
            location,
            todos: results,
        }
    }

    pub fn new(file_path: Option<&str>) -> Self {
        Storage::default(file_path)
    }

    pub fn clean(&mut self) {
        self.todos = vec![];
        self.sync().expect("Unable to sync todos into file");
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
        self.sync().expect("Unable to sync todos into file");
    }

    pub fn sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create(&self.location)?;
        let json = serde_json::to_string(&self.todos)?;
        file.write_all(json.as_bytes())?;

        Ok({})
    }

    pub fn pretty_print(&self) {
        for todo in &self.todos {
            todo.pretty_print()
        }
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }
}
