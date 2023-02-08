use rocket::serde::{Deserialize, Deserializer, Serialize};

// Probably temporary
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Title {
    rendered: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Links {
    #[serde(alias = "self", deserialize_with = "from_vec_link")]
    self_href: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Href {
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Manga {
    pub id: usize,
    pub slug: String,
    pub link: String,
    #[serde(deserialize_with = "from_title")]
    pub title: String,
    pub categories: Box<[usize]>,
    #[serde(alias = "_links")]
    pub links: Links,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct MangaImage {
    pub page_title: String,
    pub src: String,
}

fn from_title<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Title = Deserialize::deserialize(deserializer)?;

    Ok(s.rendered)
}

fn from_vec_link<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<Href> = Deserialize::deserialize(deserializer)?;

    Ok(s.first().unwrap().href.to_owned())
}
