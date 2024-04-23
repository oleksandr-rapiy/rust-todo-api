pub struct Todo {
    id: i32, 
    title: String, 
    todo_status: TodoStatus,
    user_id: i32
}

pub enum TodoStatus {
    New,
    Pending,
    Completed
}

impl Todo {
    pub fn new(title: String, user_id: i32) -> Todo {
        Todo {
            id: 0,
            title,
            todo_status: TodoStatus::New,
            user_id
        }
    }

    pub fn mark_as_completed(&mut self) {
        self.todo_status = TodoStatus::Completed;
    }
}
