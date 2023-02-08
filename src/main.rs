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

#[get("/<search>?<per_page>&<order>")]
async fn search(
    search: &str,
    per_page: Option<usize>,
    order: Option<scraper::mangayabu::Order>,
) -> Json<Vec<Manga>> {
    let a = scraper::mangayabu::MangayabuScraper::default().set_options(
        scraper::mangayabu::RequestParams {
            per_page: per_page.unwrap_or(5),
            order: order.unwrap_or_default(),
        },
    );

    let result = a.search(search).await;

    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/search", routes![search])
}
