#[macro_use]
extern crate rocket;

pub mod models;
pub mod routes;
pub mod scraper;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
pub fn hello() -> &'static str {
    "It works!"
}

#[shuttle_service::main]
pub async fn rocket() -> shuttle_service::ShuttleRocket {
    let rocket = rocket::build()
        .attach(CORS)
        .mount("/", routes![hello])
        .mount(
            "/mangayabu",
            routes![
                routes::mangayabu::search,
                routes::mangayabu::manga_by_id,
                routes::mangayabu::images_by_id
            ],
        )
        .mount(
            "/goldenmanga",
            routes![
                routes::goldenmanga::search,
                routes::goldenmanga::get_manga,
                routes::goldenmanga::get_manga_images
            ],
        );

    Ok(rocket)
}
