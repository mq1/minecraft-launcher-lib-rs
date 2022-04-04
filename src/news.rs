use anyhow::Result;
use isahc::{ReadResponseExt, Request, RequestExt};
use serde::Deserialize;
use url::Url;

use crate::MINECRAFT_NET_URL;

const ARTICLES_URL: &str =
    "https://www.minecraft.net/content/minecraft-net/_jcr_content.articles.grid";

#[derive(Deserialize)]
pub struct Image {
    pub content_type: String,

    #[serde(rename(deserialize = "imageURL"))]
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct Tile {
    pub sub_header: String,
    pub image: Image,
    pub tile_size: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct Article {
    pub default_tile: Tile,
    #[serde(rename(deserialize = "articleLang"))]
    pub article_lang: String,
    pub primary_category: String,
    pub preferred_tile: Option<Tile>,
    pub categories: Vec<String>,
    pub article_url: String,
    pub publish_date: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize)]
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

    let mut resp = Request::get(url.to_string())
        .header("user-agent", user_agent)
        .body(())?
        .send()
        .expect("Failed getting articles.grid");

    let mut articles = resp
        .json::<Articles>()
        .expect("Failed parsing articles.grid");

    // set complete URLs
    for article in articles.article_grid.iter_mut() {
        let image_url =
            Url::parse(MINECRAFT_NET_URL)?.join(&article.default_tile.image.image_url)?;
        article.default_tile.image.image_url = image_url.to_string();

        let article_url = Url::parse(MINECRAFT_NET_URL)?.join(&article.article_url)?;
        article.article_url = article_url.to_string();
    }

    Ok(articles)
}
