use crate::models::manga::{Manga, MangaImage, Order};
use crate::scraper::mangayabu::{MangayabuScraper, RequestParams};

use rocket::response::status::NotFound;
use rocket::serde::json::Json;

#[get("/search/<search>?<per_page>&<order>")]
pub async fn search(
    search: &str,
    per_page: Option<usize>,
    order: Option<Order>,
) -> Result<Json<Vec<Manga>>, NotFound<String>> {
    let manga_scraper = MangayabuScraper::default().set_options(RequestParams {
        per_page: per_page.unwrap_or(5),
        order: order.unwrap_or_default(),
    });

    let result = manga_scraper.search(search).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}

#[get("/<id>?<per_page>&<order>")]
pub async fn manga_by_id(
    id: usize,
    per_page: Option<usize>,
    order: Option<Order>,
) -> Result<Json<Manga>, NotFound<String>> {
    let manga_scraper = MangayabuScraper::default().set_options(RequestParams {
        per_page: per_page.unwrap_or(5),
        order: order.unwrap_or_default(),
    });

    let result = manga_scraper.get_manga_by_id(id).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}

#[get("/images/<id>?<per_page>&<order>")]
pub async fn images_by_id(
    id: usize,
    per_page: Option<usize>,
    order: Option<Order>,
) -> Result<Json<Vec<MangaImage>>, NotFound<String>> {
    let manga_scraper = MangayabuScraper::default().set_options(RequestParams {
        per_page: per_page.unwrap_or(5),
        order: order.unwrap_or_default(),
    });

    let result = manga_scraper.get_images_by_id(id).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}
