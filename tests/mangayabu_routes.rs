#[macro_use]
extern crate rocket;

#[cfg(test)]
mod mangayabu_tests {
    use manga_suku::models::manga::Order;
    use manga_suku::{hello, routes};
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    pub fn ignite() -> rocket::Rocket<rocket::Build> {
        rocket::build().mount("/", routes![hello]).mount(
            "/mangayabu",
            routes![
                routes::mangayabu::search,
                routes::mangayabu::manga_by_id,
                routes::mangayabu::images_by_id
            ],
        )
    }

    #[test]
    fn it_works() {
        let client = Client::tracked(ignite()).expect("valid rocket instance");
        let response = client.get(uri!(manga_suku::hello)).dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "It works!");
    }

    #[test]
    fn search_manga() {
        let client = Client::tracked(ignite()).expect("valid rocket instance");

        let uri = uri!(
            "/mangayabu",
            routes::mangayabu::search("the beginning after the end", Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn search_nonexistent_manga() {
        let client = Client::tracked(ignite()).expect("valid rocket instance");

        let uri = uri!(
            "/mangayabu",
            routes::mangayabu::search(
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
        let client = Client::tracked(ignite()).expect("valid rocket instance");

        let uri = uri!(
            "/mangayabu",
            routes::mangayabu::manga_by_id(118967, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_nonexistent_manga_by_id() {
        let client = Client::tracked(ignite()).expect("valid rocket instance");

        let uri = uri!(
            "/mangayabu",
            routes::mangayabu::manga_by_id(1, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn get_images_by_manga_id() {
        let client = Client::tracked(ignite()).expect("valid rocket instance");

        let uri = uri!(
            "/mangayabu",
            routes::mangayabu::images_by_id(118967, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_nonexistent_images_by_manga_id() {
        let client = Client::tracked(ignite()).expect("valid rocket instance");

        let uri = uri!(
            "/mangayabu",
            routes::mangayabu::images_by_id(1, Some(1), Some(Order::Asc))
        );

        let response = client.get(uri).dispatch();

        assert_eq!(response.status(), Status::NotFound);
    }
}
