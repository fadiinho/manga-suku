use crate::scraper::mangalivre::{
    MangaLivreChapter, MangaLivreManga, MangaLivrePages, MangaLivreScraper,
};

use rocket::response::status::NotFound;
use rocket::serde::json::Json;

#[get("/search/<search>")]
pub async fn search(search: &str) -> Result<Json<Vec<MangaLivreManga>>, NotFound<String>> {
    let manga_scraper = MangaLivreScraper::default();

    let result = manga_scraper.search(search).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err()));
    }

    Ok(Json(result.unwrap()))
}

#[get("/<id>/<page>")]
pub async fn get_chapters(
    id: usize,
    page: usize,
) -> Result<Json<Vec<MangaLivreChapter>>, NotFound<String>> {
    let manga_scraper = MangaLivreScraper::default();

    let result = manga_scraper.get_chapters(id, page).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}

#[get("/images/<id>")]
pub async fn get_pages(id: usize) -> Result<Json<Vec<MangaLivrePages>>, NotFound<String>> {
    let manga_scraper = MangaLivreScraper::default();

    let result = manga_scraper.get_pages(id).await;

    if result.is_err() {
        return Err(NotFound(result.unwrap_err().to_string()));
    }

    Ok(Json(result.unwrap()))
}
