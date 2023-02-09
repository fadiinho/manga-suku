mod models;
mod routes;
mod scraper;

#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "It works!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello]).mount(
        "/manga",
        routes![
            routes::manga::search,
            routes::manga::manga_by_id,
            routes::manga::images_by_id
        ],
    )
}
