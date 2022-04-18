use anyhow::Result;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use url::Url;

use crate::MINECRAFT_NET_URL;

const ARTICLES_URL: &str =
    "https://www.minecraft.net/content/minecraft-net/_jcr_content.articles.grid";

fn from_relative_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let relative_url: String = Deserialize::deserialize(deserializer)?;

    let mut url = Url::parse(MINECRAFT_NET_URL).map_err(D::Error::custom)?;
    url.set_path(&relative_url);

    Ok(url)
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub content_type: String,

    #[serde(rename(deserialize = "imageURL"))]
    #[serde(deserialize_with = "from_relative_url")]
    pub image_url: Url,
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub sub_header: String,
    pub image: Image,
    pub tile_size: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub default_tile: Tile,
    #[serde(rename(deserialize = "articleLang"))]
    pub article_lang: String,
    pub primary_category: String,
    pub preferred_tile: Option<Tile>,
    pub categories: Vec<String>,
    #[serde(deserialize_with = "from_relative_url")]
    pub article_url: Url,
    pub publish_date: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Articles {
    pub article_grid: Vec<Article>,
    pub article_count: usize,
}

/// Get the news from minecraft.net
pub fn get_minecraft_news(page_size: Option<usize>) -> Result<Articles> {
    let page_size = if page_size.is_none() {
        20
    } else {
        page_size.unwrap()
    };

    let mut url = Url::parse(ARTICLES_URL)?;
    url.query_pairs_mut()
        .append_pair("pageSize", &format!("{page_size}"));

    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let articles = ureq::get(url.as_str())
        .set("user-agent", &user_agent)
        .call()?
        .into_json()?;

    Ok(articles)
}
