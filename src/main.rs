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
        "/mangayabu",
        routes![
            routes::mangayabu::search,
            routes::mangayabu::manga_by_id,
            routes::mangayabu::images_by_id
        ],
    )
}

#[cfg(test)]
mod api_tests {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn it_works() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(super::hello)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "It works!");
    }

    #[test]
    fn search_manga() {
        use crate::models::manga::Order;

        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let uri = uri!(
            "/manga",
            super::routes::mangayabu::search(
                "the beginning after the end",
                Some(1),
                Some(Order::Asc)
            )
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn search_nonexistent_manga() {
        use crate::models::manga::Order;

        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let uri = uri!(
            "/manga",
            super::routes::mangayabu::search(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                Some(1),
                Some(Order::Asc)
            )
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn get_manga_by_id() {
        use crate::models::manga::Order;

        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let uri = uri!(
            "/manga",
            super::routes::mangayabu::manga_by_id(118967, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_nonexistent_manga_by_id() {
        use crate::models::manga::Order;

        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let uri = uri!(
            "/manga",
            super::routes::mangayabu::manga_by_id(1, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn get_images_by_manga_id() {
        use crate::models::manga::Order;

        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let uri = uri!(
            "/manga",
            super::routes::mangayabu::images_by_id(118967, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_nonexistent_images_by_manga_id() {
        use crate::models::manga::Order;

        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let uri = uri!(
            "/manga",
            super::routes::mangayabu::images_by_id(1, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}
