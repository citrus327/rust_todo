use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub name: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(name: &str, completed: bool) -> Self {
        return Self {
            name: name.to_string(),
            completed,
        };
    }

    pub fn pretty_print(&self) {
        println!("Name: {}, Completed: {}", self.name, self.completed)
    }
}
