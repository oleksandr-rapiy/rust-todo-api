#[macro_use]
extern crate rocket;
use rocket::response::Redirect;
use rocket_okapi::{openapi_get_routes, swagger_ui::*};

mod domain;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let todo_routes = openapi_get_routes![
        routes::todo::get_all,
        routes::todo::get_by_id,
        routes::todo::create,
        routes::todo::update,
        routes::todo::delete
    ];

    let _rocket = rocket::build()
        .mount(
            "/api/",
            todo_routes,
        )
        .mount("/swagger/", make_swagger_ui(&SwaggerUIConfig{
            url: "../api/openapi.json".to_string(),
            ..Default::default()
        }))
        .mount("/", routes![to_swagger])
        .launch()
        .await?;

    return Ok(());
}

#[get("/")]
fn to_swagger() -> Redirect {
    Redirect::to("/swagger")
}