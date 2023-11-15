use crate::todos::Todo;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf};
use std::{env, fs};

pub struct Storage {
    pub todos: Vec<Todo>,
    pub location: PathBuf,
}

impl Storage {
    pub fn default(file_path: Option<&str>) -> Self {
        let cwd = env::current_dir().unwrap();
        let location: PathBuf;
        if let Some(path) = file_path {
            location = cwd.join(path);
        } else {
            location = cwd.join("todo.json");
            println!("Using default storage location, {}", location.to_string_lossy());
        }

        let mut result: Vec<Todo> = vec![];
        if let Ok(_) = File::open(&location) {
            if let Ok(content) = fs::read_to_string(&location) {
                result = serde_json::from_str(content.as_str()).expect("Todo is read");
            };
        } else {
            let _ = File::create(&location);
            result = vec![];
        }

        Self {
            location: location,
            todos: result,
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
        self.sync().expect("Sync correctly");
    }

    pub fn sync(&self) -> std::io::Result<()> {
        let json = serde_json::to_string(&self.todos)?;
        let mut file = File::create(&self.location)?;
        file.write_all(json.as_bytes())
    }

    pub fn list(&self) {
        for todo in &self.todos {
            todo.pretty_print()
        }
    }
}
