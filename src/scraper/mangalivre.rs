use std::collections::HashMap;

use reqwest::{header::HeaderMap, Client};
use rocket::serde::{Deserialize, Serialize};

const BASE_URL: &'static str = "https://mangalivre.net";

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MangaLivreMangaWrapper {
    series: Vec<MangaLivreManga>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct MangaLivreCategories {
    name: String,
    id_category: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MangaLivreManga {
    #[serde(rename(serialize = "serieId", deserialize = "id_serie"))]
    id_serie: usize,
    name: String,
    cover: String,
    link: String,
    categories: Vec<MangaLivreCategories>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MangaLivreChapterWrapper {
    chapters: Vec<MangaLivreChapter>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct MangaLivreRelease {
    #[serde(rename(serialize = "releaseId", deserialize = "id_release"))]
    id_release: usize,
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MangaLivreChapter {
    #[serde(rename(serialize = "serieId", deserialize = "id_serie"))]
    id_serie: usize,
    #[serde(rename(serialize = "chapterId", deserialize = "id_chapter"))]
    id_chapter: usize,
    name: String,
    number: String,
    releases: HashMap<String, MangaLivreRelease>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MangaLivrePagesWrapper {
    images: Vec<MangaLivrePages>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MangaLivrePages {
    legacy: String,
    avif: String,
}

pub struct MangaLivreScraper {
    pub base_url: &'static str,
    client: Client,
    headers: HeaderMap,
}

impl Default for MangaLivreScraper {
    fn default() -> Self {
        let mut headers = HeaderMap::new();

        headers.insert("accept-language", "pt-BR".parse().unwrap());
        headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.5359.62 Safari/537.36".parse().unwrap());
        headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
        headers.insert(
            "content-type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        Self::new(BASE_URL, headers)
    }
}

impl MangaLivreScraper {
    pub fn new(base_url: &'static str, headers: HeaderMap) -> Self {
        MangaLivreScraper {
            base_url,
            client: Client::new(),
            headers,
        }
    }

    pub async fn search(&self, search_term: &str) -> Result<Vec<MangaLivreManga>, String> {
        let url = format!("{}{}", self.base_url, "/lib/search/series.json");

        let mut params = HashMap::new();
        params.insert("search", search_term);

        let response = self
            .client
            .post(url)
            .form(&params)
            .headers(self.headers.clone())
            .send()
            .await;

        if response.is_err() {
            return Err(response.unwrap_err().to_string());
        }

        let json_response: Result<MangaLivreMangaWrapper, _> = response.unwrap().json().await;

        if json_response.is_err() {
            return Err("Manga not found!".to_owned());
        }

        return Ok(json_response.unwrap().series);
    }

    pub async fn get_chapters(
        &self,
        id: usize,
        page: usize,
    ) -> Result<Vec<MangaLivreChapter>, String> {
        let url = format!(
            "{}/series/chapters_list.json?page={page}&id_serie={id}",
            self.base_url
        );

        let response = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await;

        if response.is_err() {
            return Err(response.unwrap_err().to_string());
        }

        let json_response: Result<MangaLivreChapterWrapper, _> = response.unwrap().json().await;

        if json_response.is_err() {
            return Err("Manga not found!".to_owned());
        }

        return Ok(json_response.unwrap().chapters);
    }

    pub async fn get_pages(&self, release_id: usize) -> Result<Vec<MangaLivrePages>, String> {
        let url = format!("{}/leitor/pages/{release_id}.json", self.base_url);

        let response = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await;

        if response.is_err() {
            return Err(response.unwrap_err().to_string());
        }

        let json_response: Result<MangaLivrePagesWrapper, _> = response.unwrap().json().await;

        if json_response.is_err() {
            return Err("Manga not found!".to_owned());
        }

        return Ok(json_response.unwrap().images);
    }
}
