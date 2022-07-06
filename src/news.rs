use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

lazy_static! {
    static ref ARTICLES_URL: Url =
        Url::parse("https://www.minecraft.net/content/minecraft-net/_jcr_content.articles.grid")
            .unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Articles {
    #[serde(rename = "article_grid")]
    pub article_grid: Vec<ArticleGrid>,

    #[serde(rename = "article_count")]
    pub article_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleGrid {
    #[serde(rename = "default_tile")]
    pub default_tile: Tile,

    #[serde(rename = "articleLang")]
    pub article_lang: ArticleLang,

    #[serde(rename = "primary_category")]
    pub primary_category: String,

    #[serde(rename = "categories")]
    pub categories: Vec<String>,

    #[serde(rename = "article_url")]
    pub article_url: String,

    #[serde(rename = "publish_date")]
    pub publish_date: String,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "preferred_tile")]
    pub preferred_tile: Option<Tile>,
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    #[serde(rename = "sub_header")]
    pub sub_header: String,

    #[serde(rename = "image")]
    pub image: Image,

    #[serde(rename = "tile_size")]
    pub tile_size: TileSize,

    #[serde(rename = "title")]
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "content_type")]
    pub content_type: ContentType,

    #[serde(rename = "imageURL")]
    pub image_url: String,

    #[serde(rename = "alt")]
    pub alt: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ArticleLang {
    #[serde(rename = "en-us")]
    EnUs,
}

#[derive(Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "image")]
    Image,
}

#[derive(Serialize, Deserialize)]
pub enum TileSize {
    #[serde(rename = "1x1")]
    The1X1,

    #[serde(rename = "2x1")]
    The2X1,

    #[serde(rename = "2x2")]
    The2X2,
}

/// Get the news from minecraft.net
pub fn get_minecraft_news(page_size: Option<usize>) -> Result<Articles> {
    let page_size = page_size.unwrap_or(20);

    let mut url = ARTICLES_URL.clone();
    url.query_pairs_mut()
        .append_pair("pageSize", &format!("{page_size}"));

    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let articles = attohttpc::get(url)
        .header("user-agent", &user_agent)
        .send()?
        .json()?;

    Ok(articles)
}
