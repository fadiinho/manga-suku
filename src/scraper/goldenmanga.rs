use reqwest::Client;
use rocket::serde::{Deserialize, Serialize};
use scraper::{Html, Selector};

const BASE_URL: &'static str = "https://goldenmangas.top";

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GoldenmangaMangaSearch {
    name: String,
    link: String,
    cover_image: String,
}

pub struct GoldenmangaScraper {
    pub base_url: &'static str,
    client: Client,
}

impl Default for GoldenmangaScraper {
    fn default() -> Self {
        Self::new(BASE_URL)
    }
}

impl GoldenmangaScraper {
    pub fn new(base_url: &'static str) -> Self {
        GoldenmangaScraper {
            base_url,
            client: Client::new(),
        }
    }

    fn get_mangas_from_search(&self, text: String) -> Vec<GoldenmangaMangaSearch> {
        let html = Html::parse_document(&text);

        let mangas_selector =
            Selector::parse("div.container > section.row > div.mangas > a").unwrap();

        let cover_selector = Selector::parse("div.MangaImagem > img").unwrap();

        let mangas_div = html.select(&mangas_selector);

        let mut mangas: Vec<GoldenmangaMangaSearch> = Vec::new();

        for a_elem in mangas_div {
            let href = a_elem.value().attr("href").unwrap();
            let name = a_elem.text().collect::<String>().replace("\n", "");
            let cover_image = a_elem
                .select(&cover_selector)
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap();

            mangas.push(GoldenmangaMangaSearch {
                name,
                link: format!("{}{}", BASE_URL, href),
                cover_image: format!("{}{}", BASE_URL, cover_image),
            });
        }

        mangas
    }

    pub async fn search(&self, search: &str) -> Result<Vec<GoldenmangaMangaSearch>, &str> {
        let url = format!("{}/mangabr?busca={}", BASE_URL, search);
        let response = self.client.get(url).send().await;

        if response.is_err() {
            return Err("Manga not found!");
        }

        let mangas = self.get_mangas_from_search(response.unwrap().text().await.unwrap());

        if mangas.is_empty() {
            return Err("Manga not found!");
        }

        Ok(mangas)
    }
}
