use crate::models::manga::Manga;

mod models;
mod scraper;

#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search/<search>")]
async fn search(search: &str) -> Json<Vec<Manga>> {
    let a = scraper::mangayabu::MangayabuScraper::default();
    let result = a.search(search).await;

    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}
