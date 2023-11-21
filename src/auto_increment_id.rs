use crate::storage::Storage;

pub fn get_newest_id(storage: &Storage) -> i32 {
  let todos = storage.get_todos();
  let ids = todos.iter().map(|o| o.id);

  let mut max = -1;

  for id in ids {
    if id > max {
      max = id
    }
  }

  max + 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_get_largest_id_in_storage() {
    let mut storage: Storage = Storage::new(None);
    storage.clean();
    storage.add("t1", false);
    storage.add("t2", false);
    storage.add("t3", false);
    storage.add("t4", false);
    let id = storage.add("t5", false);

    assert_eq!(storage.get_todos().len(), 5);
    assert_eq!(get_newest_id(&storage), id + 1);
  }
}
