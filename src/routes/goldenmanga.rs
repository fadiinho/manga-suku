use crate::scraper::goldenmanga::{GoldenmangaMangaSearch, GoldenmangaScraper};

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
