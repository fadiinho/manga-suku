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

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GoldenmangaChapter {
    link: String,
    chapter: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GoldenmangaManga {
    name: String,
    description: String,
    chapters: Vec<GoldenmangaChapter>,
    path: String,
    cover: String,
    #[serde(rename(serialize = "isNsfw", deserialize = "is_nsfw"))]
    is_nsfw: bool,
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

    pub async fn get_manga_by_path(&self, manga_path: &str) -> Result<GoldenmangaManga, &str> {
        let url = format!("{}/mangabr/{}", BASE_URL, manga_path);
        let response = self.client.get(&url).send().await;

        if response.is_err() {
            return Err("Manga not found!");
        }

        Ok(self.get_manga_info(response.unwrap().text().await.unwrap(), url)?)
    }

    fn get_manga_info(&self, text: String, url: String) -> Result<GoldenmangaManga, &str> {
        let html = Html::parse_document(&text);

        let title_selector = Selector::parse("div.row > div > h2").unwrap();
        let description_selector =
            Selector::parse("div.row > div > div#manga_capitulo_descricao").unwrap();
        let chapters_selector = Selector::parse("ul.capitulos li > a").unwrap();
        let cover_selector =
            Selector::parse(".container.manga div.row .text-right > .img-responsive").unwrap();
        let nsfw_selector = Selector::parse("#capitulos_aviso").unwrap();

        let mut title = html.select(&title_selector);
        let mut description = html.select(&description_selector);
        let mut cover = html.select(&cover_selector);
        let mut nsfw = html.select(&nsfw_selector);
        let chapters_container = html.select(&chapters_selector);

        let mut chapters: Vec<GoldenmangaChapter> = Vec::new();

        for chapter in chapters_container {
            chapters.push(GoldenmangaChapter {
                link: format!("{}{}", BASE_URL, chapter.value().attr("href").unwrap()),
                chapter: chapter
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_owned()
                    .split("\n")
                    .take(1)
                    .collect::<String>()
                    .replace("Cap ", "")
                    .trim()
                    .to_owned(),
            });
        }

        Ok(GoldenmangaManga {
            name: title.next().unwrap().text().collect::<String>(),
            description: description.next().unwrap().text().collect::<String>(),
            path: url.split('/').last().unwrap().to_owned(),
            cover: format!(
                "{}{}",
                BASE_URL,
                cover.next().unwrap().value().attr("src").unwrap()
            ),
            chapters,
            is_nsfw: nsfw.next().is_some(),
        })
    }

    fn parse_manga_images(&self, text: String) -> Vec<String> {
        let html = Html::parse_document(&text);

        let pages_selector = Selector::parse("#capitulos_images img").unwrap();

        let pages_element = html.select(&pages_selector);

        let mut pages: Vec<String> = vec![];

        for page in pages_element {
            let src = page.value().attr("src");

            if src.is_some() {
                pages.push(format!("{}{}", BASE_URL, src.unwrap().to_owned()));
            }
        }

        pages
    }

    pub async fn get_manga_images(
        &self,
        manga_path: &str,
        chapter: &str,
    ) -> Result<Vec<String>, &str> {
        let url = format!("{}/mangabr/{}/{}", BASE_URL, manga_path, chapter);
        let response = self.client.get(url).send().await;

        if response.is_err() {
            return Err("Manga not found!");
        }

        let result: Vec<String> = self.parse_manga_images(response.unwrap().text().await.unwrap());

        Ok(result)
    }
}
