#[macro_use]
extern crate rocket;

use manga_suku::rocket as rocket_run;

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket_run().await
}
