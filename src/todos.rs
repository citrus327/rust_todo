pub struct Todo {
    pub name: String,
    pub completed: bool,
}

pub struct Storage {
    pub location: String,
}

impl Storage {
    pub fn default() -> Self {
        Self {
            location: "123".to_string(),
        }
    }

    pub fn store(&self, todo: Todo) {
        println!(
            "Todo stored, name: {}!, status: {}!",
            todo.name, todo.completed
        )
    }
}

impl Todo {
    fn new(name: &str, completed: bool) -> Self {
        return Self {
            name: name.to_string(),
            completed,
        };
    }

    fn add(name: &str, completed: bool) {
        let a = Self {
            name: name.to_string(),
            completed,
        };
    }
}
