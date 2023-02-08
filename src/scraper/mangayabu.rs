use std::fmt::Display;

use scraper::{Html, Selector};

use reqwest;
use rocket::serde::Deserialize;

use crate::models::manga::{Manga, MangaImage};

const BASE_URL: &'static str = "https://mangayabu.top";

#[allow(dead_code)]
#[derive(Clone, Copy, FromFormField)]
pub enum Order {
    Asc,
    Desc,
}

impl Default for Order {
    fn default() -> Self {
        Order::Asc
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Order::Asc => write!(f, "asc"),
            Order::Desc => write!(f, "desc"),
        }
    }
}

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
}

impl Default for MangayabuScraper {
    fn default() -> Self {
        Self::new(BASE_URL, RequestParams::default())
    }
}

impl MangayabuScraper {
    pub fn new(base_url: &'static str, options: RequestParams) -> Self {
        Self { base_url, options }
    }

    async fn get<T>(&self, url: String) -> T
    where
        T: for<'a> Deserialize<'a>,
    {
        let response = reqwest::get(url).await.unwrap();
        let json_response: T = response.json().await.unwrap();

        json_response
    }

    fn build_url(&self, path: String) -> String {
        let url = format!("{}/{}{}", self.base_url, path, self.options.into_params());

        url
    }

    pub async fn search(&self, search: &str) -> Vec<Manga> {
        let url = &self.build_url(format!("wp-json/wp/v2/posts?search={}", search));

        let response = self.get(url.to_owned()).await;

        response
    }

    pub fn set_options(mut self, params: RequestParams) -> Self {
        self.options = params;
        return self;
    }

    pub async fn get_manga_by_id(&self, id: usize) -> Manga {
        let url = &self.build_url(format!("wp-json/wp/v2/posts/{}?", id));

        let response = self.get(url.to_owned()).await;

        response
    }

    pub async fn get_images_by_url(&self, url: String) -> Vec<MangaImage> {
        let response = reqwest::get(url).await.unwrap();
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

    pub async fn get_images_by_id(&self, id: usize) -> Vec<MangaImage> {
        let manga = self.get_manga_by_id(id).await;

        let response = self.get_images_by_url(manga.link).await;

        response
    }
}
