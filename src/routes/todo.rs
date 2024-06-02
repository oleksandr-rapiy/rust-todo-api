use rocket::form::Form;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;
use crate::domain::todo::Todo;
use crate::storage::todo_storage::ToDoStorage;


#[openapi(tag = "Todo")]
#[get("/todo")]
pub fn get_all() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Todo")]
#[get("/todo/<id>")]
pub async fn get_by_id(id: u64) -> Json<Todo> {
    let storage = ToDoStorage::new().await.unwrap();

    let todo = storage.get_by_id(&id).await.unwrap();

    return Json(todo);
}

#[openapi(tag = "Todo")]
#[post("/todo", format = "application/json", data = "<todo>")]
pub async fn create(todo: Json<Todo>) -> Json<Todo> {
    let storage = ToDoStorage::new().await.unwrap();
    let todo = storage.create(todo.0).await.unwrap();

    return Json(todo);
}

#[openapi(tag = "Todo")]
#[put("/todo/<id>", format = "application/json", data = "<todo>")]
pub fn update(id: i32, todo: String) -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Todo")]
#[delete("/todo/<id>")]
pub fn delete(id: i32) -> &'static str {
    "Hello, world!"
}
