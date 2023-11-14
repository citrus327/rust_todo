mod storage;
mod todos;

#[cfg(test)]
mod tests {
    use crate::todos::Todo;

    use super::*;
    use storage::Storage;

    #[test]
    fn it_works() {
        let mut storage = Storage::default();
        let _ = &storage.add(Todo::new("New Todo", false));
        let _ = &storage.add(Todo::new("New Todo", false));
        // let _ = &storage.add(Todo::new("New Todo", false));
        // let _ = &storage.add(Todo::new("New Todo", false));
        // let _ = &storage.add(Todo::new("New Todo", false));

        storage.list();
    }
}
