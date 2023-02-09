use rocket::http::impl_from_uri_param_identity;
use rocket::http::uri::fmt::{Query, UriDisplay};
use rocket::serde::{Deserialize, Deserializer, Serialize};
use std::fmt::Display;

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

#[derive(Copy, Clone, FromFormField)]
#[allow(dead_code)]
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

impl UriDisplay<Query> for Order {
    fn fmt(&self, f: &mut rocket::http::uri::fmt::Formatter<'_, Query>) -> std::fmt::Result {
        match self {
            Order::Asc => f.write_named_value("order", "asc"),
            Order::Desc => f.write_named_value("order", "desc"),
        }
    }
}

impl_from_uri_param_identity!([Query] Order);
