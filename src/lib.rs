#[macro_use]
extern crate rocket;

pub mod models;
pub mod routes;
pub mod scraper;

#[get("/")]
pub fn hello() -> &'static str {
    "It works!"
}
