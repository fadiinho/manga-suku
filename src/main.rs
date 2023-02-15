#[macro_use]
extern crate rocket;

use manga_suku::{hello, routes};

#[launch]
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
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
        )
}
