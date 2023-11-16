use crate::todos::Todo;
use std::fs::File;
use std::io::prelude::*;
use std::path::{PathBuf, Path};
use std::{env, fs};

pub struct Storage {
    pub todos: Vec<Todo>,
    pub location: PathBuf,
}

fn read_todos_from_file (location: &PathBuf) -> Result<Vec<Todo>, serde_json::Error>{
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
    pub fn default(file_path: Option<&str>) -> Self {
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

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
        self.sync().expect("Sync correctly");
    }

    pub fn sync(&self) -> std::io::Result<()> {
        println!("location is {}", self.location.to_string_lossy());
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
