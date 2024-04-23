use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

#[openapi(tag = "Todo")]
#[get("/todo")]
pub fn get_all() -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Todo")]
#[get("/todo/<id>")]
pub fn get_by_id(id: i32) -> &'static str {
    "Hello, world!"
}

#[openapi(tag = "Todo")]
#[post("/todo", format = "application/json", data = "<todo>")]
pub fn create(todo: String) -> &'static str {
    "Hello, world!"
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
