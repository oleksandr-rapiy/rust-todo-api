use darkbird::{Options, Storage, StorageType};
use rand;

use crate::domain::todo::Todo;

pub struct ToDoStorage<'a> {
    path: &'a str,
    name: &'a str,
    size: usize,
    storage_type: StorageType,
    storage: Storage<Id, Todo>,
}

type Id = u64;

impl ToDoStorage<'_> {
    const PATH: &'static str = ".";
    const NAME: &'static str = "todo_storage";
    const SIZE: usize = 1024;

    pub async fn new() -> Option<Self> {
        let ops = Options::new(
            Self::PATH,
            Self::NAME,
            Self::SIZE,
            StorageType::RamCopies,
            true,
        );

        let storage = match Storage::<Id, Todo>::open(ops).await {
            Ok(storage) => storage,
            Err(err) => {
                eprintln!("Error: {}", err);
                return None;
            }
        };

        Some(ToDoStorage {
            path: Self::PATH,
            name: Self::NAME,
            size: Self::SIZE,
            storage_type: StorageType::RamCopies,
            storage,
        })
    }

    pub async fn get_by_id(&self, id: &u64) -> Option<Todo> {
        let todo = self.storage.lookup(id);

        match todo {
            Some(todo_ref) => Some(todo_ref.clone()),
            None => None,
        }
    }

    pub fn get() {}

    pub async fn create(&self, todo: Todo) -> Option<Todo> {
        match self.storage.insert(todo.id, todo.clone()).await {
            Ok(_) => Some(todo),
            Err(_) => None,
        }
    }

    pub fn delete(id: i32) {}

    pub fn update() {}
}
