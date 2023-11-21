use std::{error::Error, fs, path::PathBuf};

use directories::ProjectDirs;

pub fn make_todo_file() -> PathBuf {
  if is_todo_file_created() {
    return get_todo_file_path().unwrap();
  }

  let project_dir =
    ProjectDirs::from("", "", env!("CARGO_PKG_NAME").to_uppercase().as_str()).unwrap();
  let dir_path = project_dir.config_dir();
  fs::create_dir_all(dir_path).unwrap();

  let todo_path = dir_path.join("todo.json");

  fs::write(&todo_path, "[]").unwrap_or_else(|e| {
    panic!(
      "Unable to create data file at {}, error: {}",
      &todo_path.to_string_lossy(),
      e
    );
  });

  println!("File created at, {}", &todo_path.to_string_lossy());

  todo_path
}

pub fn get_todo_file_path() -> Result<PathBuf, Box<dyn Error>> {
  let project_dir =
    ProjectDirs::from("", "", env!("CARGO_PKG_NAME").to_uppercase().as_str()).unwrap();
  let dir_path = project_dir.config_dir();
  let todo_path = dir_path.join("todo.json");

  Ok(todo_path)
}

pub fn is_todo_file_created() -> bool {
  let todo_file_path = get_todo_file_path().unwrap();
  let result = fs::read_to_string(todo_file_path);
  result.is_ok()
}

#[cfg(test)]
mod tests {
  use std::fs::{self};

  use crate::dir::get_todo_file_path;

  use super::{is_todo_file_created, make_todo_file};

  #[test]
  fn it_should_create_data_file() {
    make_todo_file();
    assert_eq!(is_todo_file_created(), true);
  }

  #[test]
  fn remove_data_file() {
    make_todo_file();
    fs::remove_file(get_todo_file_path().unwrap()).expect("Failed to remove file");
    let result = fs::read_to_string(get_todo_file_path().unwrap());
    assert_eq!(result.is_err(), true);
  }
}
