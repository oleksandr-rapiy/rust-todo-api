use darkbird::document::{self, RangeField};
use rand;
use rand::Rng;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, FromForm, JsonSchema)]
pub struct Todo {
    pub  id: u64,
    pub  title: String,
    pub  is_completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            id: rand::thread_rng().gen_range(0..u64::MAX),
            title,
            is_completed: false
        }
    }

    pub fn mark_as_completed(&mut self) {
        self.is_completed = true;
    }
}

impl document::Document for Todo {}

impl document::Indexer for Todo {
    fn extract(&self) -> Vec<String> {
        vec![]
    }
}

impl document::Tags for Todo {
    fn get_tags(&self) -> Vec<String> {
        vec![]
    }
}

impl document::Range for Todo {
    fn get_fields(&self) -> Vec<RangeField> {
        vec![]
    }
}

impl document::MaterializedView for Todo {
    fn filter(&self) -> Option<String> {
        None
    }
}


impl document::FullText for Todo {
    fn get_content(&self) -> Option<String> {
        None
    }
}
