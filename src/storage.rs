use prettytable::{row, Cell, Row, Table};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs};
extern crate prettytable;

use crate::auto_increment_id::get_newest_id;
use crate::dir::get_todo_file_path;

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

#[derive(Serialize, Deserialize)]
pub struct Todo {
  pub id: i32,
  pub name: String,
  pub completed: bool,
}

impl Todo {
  pub fn new(id: i32, name: &str, completed: bool) -> Self {
    Self {
      id,
      name: name.to_string(),
      completed,
    }
  }
}

pub struct Storage {
  todos: Vec<Todo>,
  location: PathBuf,
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

  fn sync(&self) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(&self.location)?;
    let json = serde_json::to_string(&self.todos)?;
    file.write_all(json.as_bytes())?;

    Ok({})
  }

  pub fn pretty_print(&self) {
    println!(
      "To-do file location: {:?}",
      get_todo_file_path().unwrap().to_string_lossy()
    );
    let rows: Vec<Row> = self
      .todos
      .iter()
      .map(|o| {
        Row::new(vec![
          Cell::new(o.id.to_string().as_str()),
          Cell::new(o.name.as_str()),
          Cell::new(if o.completed { "Done" } else { "Not Completed" }),
        ])
      })
      .collect();

    let mut table = Table::new();

    table.add_row(row!["ID", "Todo Name", "Status"]);
    // Add a row per time
    for row in rows {
      table.add_row(row);
    }

    // Print the table to stdout
    table.printstd();
  }

  pub fn get_todos(&self) -> &Vec<Todo> {
    &self.todos
  }

  pub fn add(&mut self, name: &str, completed: bool) -> i32 {
    let id = get_newest_id(&self);
    let todo = Todo::new(id, name, completed);
    self.todos.push(todo);

    self.sync().expect("Unable to sync todos into file");

    println!("{} is added to storage, id: {}", name, id);

    id
  }

  pub fn get_by_id(&self, id: i32) -> &Todo {
    let result = self
      .todos
      .iter()
      .find(|o| o.id == id)
      .expect("Target To-Do Not found!");

    result
  }

  pub fn complete(&mut self, id: i32) {
    self.update(id, |o| o.completed = true);
    self.sync().expect("Unable to sync todos into file");
  }

  pub fn uncomplete(&mut self, id: i32) {
    self.update(id, |o| o.completed = false);
    self.sync().expect("Unable to sync todos into file");
  }

  pub fn edit_name(&mut self, id: i32, name: &str) {
    self.update(id, |o| o.name = name.to_string());
    self.sync().expect("Unable to sync todos into file");
  }

  fn update<F>(&mut self, id: i32, closure: F)
  where
    F: FnOnce(&mut Todo),
  {
    let todo = self
      .todos
      .iter_mut()
      .find(|o| o.id == id)
      .expect("Target To-Do Not found!");
    closure(todo)
  }
}

#[cfg(test)]
mod tests {
  use super::Storage;

  fn prepare() -> Storage {
    let mut storage = Storage::new(None);
    storage.clean();

    storage
  }

  #[test]
  fn it_should_add_to_storage() {
    let mut storage = prepare();
    storage.pretty_print();
    storage.add("New Todo", false);
    storage.add("New Todo", false);
    storage.add("New Todo", false);
    storage.add("New Todo", false);
    assert_eq!(storage.get_todos().len(), 4);
  }

  #[test]
  fn it_should_clean_storage() {
    let mut storage = prepare();
    storage.add("New Todo", false);
    storage.add("New Todo", false);
    storage.add("New Todo", false);
    storage.add("New Todo", false);
    storage.clean();
    assert_eq!(storage.get_todos().len(), 0);
  }

  #[test]
  fn it_should_get_todo_by_id() {
    let mut storage = prepare();

    const TARGET_NAME: &str = "New Todo";
    let id = storage.add(TARGET_NAME, false);
    storage.pretty_print();

    assert_eq!(storage.get_todos().len(), 1);
    assert_eq!(TARGET_NAME, storage.get_by_id(id).name);
  }

  #[test]
  fn it_should_complete_todo_by_id() {
    let mut storage = prepare();

    let id = storage.add("New Tod222o", false);
    assert_eq!(false, storage.get_by_id(id).completed);
    storage.complete(id);
    assert_eq!(true, storage.get_by_id(id).completed);
  }

  #[test]
  fn it_should_uncomplete_todo_by_id() {
    let mut storage = prepare();
    let id = storage.add("New Tod222o", true);
    assert_eq!(true, storage.get_by_id(id).completed);
    storage.uncomplete(id);
    assert_eq!(false, storage.get_by_id(id).completed);
  }

  #[test]
  fn it_should_edit_name_by_id() {
    let mut storage = prepare();

    const ORIGINAL_NAME: &str = "New Todo";
    const NEW_NAME: &str = "Todo New";
    let id = storage.add(ORIGINAL_NAME, true);
    assert_eq!(ORIGINAL_NAME, storage.get_by_id(id).name);
    storage.edit_name(id, NEW_NAME);
    assert_eq!(NEW_NAME, storage.get_by_id(id).name);
  }

  #[test]
  fn it_should_pretty_print() {
    let mut storage = prepare();
    storage.add("Finish reading", false);
    storage.add("Finish rust todo", false);
    storage.add("dinner", false);
    storage.add("breakfast", false);

    storage.pretty_print();
  }
}
