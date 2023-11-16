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
        storage.add(Todo::new("New Todo1", false));
        storage.add(Todo::new("New Todo2", false));
        storage.add(Todo::new("New Todo3", false));
        storage.add(Todo::new("New Todo4", false));
        storage.add(Todo::new("New Todo5", false));
        storage.list();
    }

    #[test]
    fn option () {
        let num = if let Some(_num) = Some(6) {
            5_i32
        } else {
            6_i32
        };

        assert_eq!(num, 5);
    }
}
