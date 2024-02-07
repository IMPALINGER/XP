use rocket::{get, routes};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use rocket::serde::json::serde_json::json;

#[get("/")]
fn index() -> Template {
    Template::render("index", json!({
        "none": "none",
    }))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", FileServer::from(relative!("/static")))
        .mount("/", routes![index])
        .attach(Template::fairing());

    Ok(rocket.into())
}
