use std::fmt::Display;

use reqwest;

use crate::models::manga::Manga;

const BASE_URL: &'static str = "https://mangayabu.top";

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Order {
    Asc,
    Desc,
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
    pub per_page: u32,
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
    pub options: RequestParams,
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

    fn build_url(&self, path: String) -> String {
        let url = format!("{}/{}{}", self.base_url, path, self.options.into_params());

        url
    }

    pub async fn search(&self, search: &str) -> Vec<Manga> {
        let url = &self.build_url(format!("wp-json/wp/v2/posts?search={}", search));
        let response = reqwest::get(url).await.unwrap();
        let json_response: Vec<Manga> = response.json().await.unwrap();

        json_response
    }
}
