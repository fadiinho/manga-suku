use crate::models::manga::{Manga, MangaImage};
use crate::scraper::mangayabu::{MangayabuScraper, Order, RequestParams};

use rocket::serde::json::Json;

#[get("/search/<search>?<per_page>&<order>")]
pub async fn search(
    search: &str,
    per_page: Option<usize>,
    order: Option<Order>,
) -> Json<Vec<Manga>> {
    let manga_scraper = MangayabuScraper::default().set_options(RequestParams {
        per_page: per_page.unwrap_or(5),
        order: order.unwrap_or_default(),
    });

    let result = manga_scraper.search(search).await;

    Json(result)
}

#[get("/<id>?<per_page>&<order>")]
pub async fn manga_by_id(id: usize, per_page: Option<usize>, order: Option<Order>) -> Json<Manga> {
    let manga_scraper = MangayabuScraper::default().set_options(RequestParams {
        per_page: per_page.unwrap_or(5),
        order: order.unwrap_or_default(),
    });

    let result = manga_scraper.get_manga_by_id(id).await;

    Json(result)
}

#[get("/images/<id>?<per_page>&<order>")]
pub async fn images_by_id(
    id: usize,
    per_page: Option<usize>,
    order: Option<Order>,
) -> Json<Vec<MangaImage>> {
    let manga_scraper = MangayabuScraper::default().set_options(RequestParams {
        per_page: per_page.unwrap_or(5),
        order: order.unwrap_or_default(),
    });

    let result = manga_scraper.get_images_by_id(id).await;

    Json(result)
}
