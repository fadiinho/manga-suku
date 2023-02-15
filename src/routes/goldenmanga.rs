use crate::scraper::goldenmanga::{GoldenmangaManga, GoldenmangaMangaSearch, GoldenmangaScraper};

use rocket::response::status::NotFound;
use rocket::serde::json::Json;

#[get("/search/<search>")]
pub async fn search(search: &str) -> Result<Json<Vec<GoldenmangaMangaSearch>>, NotFound<String>> {
    let manga_scraper = GoldenmangaScraper::default();

    let result = manga_scraper.search(search).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}

#[get("/<manga_path>")]
pub async fn get_manga(manga_path: &str) -> Result<Json<GoldenmangaManga>, NotFound<String>> {
    let manga_scraper = GoldenmangaScraper::default();

    let result = manga_scraper.get_manga_by_path(manga_path).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}

#[get("/images/<manga_path>?<chapter>")]
pub async fn get_manga_images(
    manga_path: &str,
    chapter: &str,
) -> Result<Json<Vec<String>>, NotFound<String>> {
    let manga_scraper = GoldenmangaScraper::default();

    let result = manga_scraper.get_manga_images(manga_path, chapter).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}
