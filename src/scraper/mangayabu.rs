use scraper::{Html, Selector};

use reqwest::Client;
use rocket::serde::Deserialize;

use crate::models::manga::{Manga, MangaImage, Order};

const BASE_URL: &'static str = "https://mangayabu.top";

#[derive(Clone, Copy)]
pub struct RequestParams {
    pub per_page: usize,
    pub order: Order,
}

impl RequestParams {
    pub fn into_params(&self) -> String {
        format!("&per_page={}&order={}", self.per_page, self.order)
    }
}

impl Default for RequestParams {
    fn default() -> Self {
        Self {
            per_page: 5,
            order: Order::Asc,
        }
    }
}

pub struct MangayabuScraper {
    pub base_url: &'static str,
    options: RequestParams,
    client: Client,
}

impl Default for MangayabuScraper {
    fn default() -> Self {
        Self::new(BASE_URL, RequestParams::default())
    }
}

impl MangayabuScraper {
    pub fn new(base_url: &'static str, options: RequestParams) -> Self {
        Self {
            base_url,
            options,
            client: Client::new(),
        }
    }

    async fn get<T>(&self, url: String) -> Result<T, &str>
    where
        T: for<'a> Deserialize<'a>,
    {
        let response = self.client.get(url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
            .send().await.unwrap();

        if !response.status().is_success() {
            return Err("Manga not found!");
        }

        let json_response: T = response.json().await.unwrap();

        Ok(json_response)
    }

    fn build_url(&self, path: String) -> String {
        let url = format!("{}/{}{}", self.base_url, path, self.options.into_params());

        url
    }

    pub async fn search(&self, search: &str) -> Result<Vec<Manga>, &str> {
        let url = &self.build_url(format!("wp-json/wp/v2/posts?search={}", search));

        let response: Result<Vec<Manga>, _> = self.get(url.to_owned()).await;

        if response.is_err() {
            return Err("Manga not Found!");
        }

        let mangas = response.unwrap();

        if mangas.is_empty() {
            return Err("Manga not Found!");
        }

        Ok(mangas)
    }

    pub fn set_options(mut self, params: RequestParams) -> Self {
        self.options = params;
        return self;
    }

    pub async fn get_manga_by_id(&self, id: usize) -> Result<Manga, &str> {
        let url = &self.build_url(format!("wp-json/wp/v2/posts/{}?", id));

        let response: Result<Manga, _> = self.get(url.to_owned()).await;

        if response.is_err() {
            return Err("Manga not Found!");
        }

        Ok(response.unwrap())
    }

    pub async fn get_images_by_url(&self, url: String) -> Vec<MangaImage> {
        let response = self.client.get(url)
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
            .send().await.unwrap();

        let html = response.text().await.unwrap();

        let document = Html::parse_document(&html);

        let images_div_selector = Selector::parse("div.manga-content img").unwrap();
        let images_div = document.select(&images_div_selector);

        let mut images: Vec<MangaImage> = Vec::new();

        for image in images_div {
            let title: String = image.value().attr("title").unwrap().into();

            let raw_src = image.value().attr("src").unwrap();
            let index = raw_src.find("https://").unwrap();
            let src: String = raw_src.chars().skip(index).collect();

            images.push(MangaImage {
                page_title: title,
                src,
            });
        }

        images
    }

    pub async fn get_images_by_id(&self, id: usize) -> Result<Vec<MangaImage>, &str> {
        let manga: Result<_, _> = self.get_manga_by_id(id).await;

        if manga.is_err() {
            return Err("Images not found!");
        }

        let response = self.get_images_by_url(manga.unwrap().link).await;

        if response.is_empty() {
            return Err("Images not found!");
        }

        Ok(response)
    }
}
