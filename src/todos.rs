use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: &str, completed: bool) -> Self {
        Self {
            name: name.to_string(),
            completed,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn pretty_print(&self) {
        println!("Name: {}, Completed: {}", self.name, self.completed)
    }
}
