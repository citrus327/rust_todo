mod storage;
mod todos;

#[cfg(test)]
mod tests {
    use crate::todos::Todo;

    use super::*;
    use storage::Storage;

    #[test]
    fn it_works() {
        let mut storage = Storage::default(None);
        let _ = &storage.add(Todo::new("New Todo1", false));
        let _ = &storage.add(Todo::new("New Todo2", false));
        let _ = &storage.add(Todo::new("New Todo3", false));
        let _ = &storage.add(Todo::new("New Todo4", false));
        let _ = &storage.add(Todo::new("New Todo5", false));

        storage.list();
    }

    fn option () {
        if let Some(num) = Some(6) {
            5 as i32
        } else {
            6 as i32
        };
    }
}
