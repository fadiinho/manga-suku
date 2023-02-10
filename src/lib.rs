pub mod models;
pub mod routes;
pub mod scraper;

#[macro_use]
extern crate rocket;

#[get("/")]
pub fn hello() -> &'static str {
    "It works!"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![hello]).mount(
        "/mangayabu",
        routes![
            routes::mangayabu::search,
            routes::mangayabu::manga_by_id,
            routes::mangayabu::images_by_id
        ],
    )
}
