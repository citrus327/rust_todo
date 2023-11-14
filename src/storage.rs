use crate::todos::Todo;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub struct Storage {
    pub todos: Vec<Todo>,
    pub location: PathBuf,
}

impl Storage {
    fn get_all_todos(&self) -> Vec<Todo> {
        let file = fs::read_to_string(&self.location).expect("The file is read");

        let result: Vec<Todo> = serde_json::from_str(&file).expect("The todo is converted");

        return result;
    }

    pub fn default() -> Self {
        let cwd = env::current_dir().unwrap();
        let cwd = cwd.join("todo.json");

        Self {
            location: cwd,
            todos: vec![],
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
        self.sync().expect("Sync correctly");
    }

    pub fn sync(&self) -> std::io::Result<()> {
        // FIXME: bad file descriptor
        let path = Path::new(&self.location);
        let json = serde_json::to_string(&self.todos)?;

        if path.exists() {
            let mut file = File::open(&self.location)?;
            file.write_all(json.as_bytes())
        } else {
            let mut file = File::create(&self.location)?;
            file.write_all(json.as_bytes())
        }
    }

    pub fn list(&self) {
        let todos = self.get_all_todos();
        for todo in todos {
            todo.pretty_print()
        }
    }
}
