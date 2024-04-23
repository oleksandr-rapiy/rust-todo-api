pub struct User {
    id: i32, 
    user_name: String
}

impl User { 
    pub fn new(user_name: String) -> User {
        User {
            id: 0, 
            user_name
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}