use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Result;
use isahc::{ReadResponseExt, Request, RequestExt};
use serde::Deserialize;
use url::Url;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
const ARTICLES_URL: &str =
    "https://www.minecraft.net/content/minecraft-net/_jcr_content.articles.grid";

/// Returns the default path to the .minecraft directory
pub fn get_minecraft_directory() -> Result<PathBuf> {
    let base_dirs = directories::BaseDirs::new().ok_or(anyhow!("BaseDirs not found"))?;

    Ok(match std::env::consts::OS {
        "windows" => base_dirs.data_dir().join(".minecraft"),
        "linux" => base_dirs.data_dir().join("minecraft"),
        _ => base_dirs.home_dir().join(".minecraft"),
    })
}

#[derive(Deserialize)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize)]
pub struct Version {
    pub id: String,
    pub r#type: String,
}

#[derive(Deserialize)]
struct VersionManifest {
    latest: LatestVersion,
    versions: Vec<Version>,
}

/// Returns the latest version of Minecraft
pub fn get_latest_version() -> Result<LatestVersion> {
    let version_manifest = isahc::get(VERSION_MANIFEST_URL)?.json::<VersionManifest>()?;

    Ok(version_manifest.latest)
}

/// Returns all versions that Mojang offers to download
pub fn get_version_list() -> Result<Vec<Version>> {
    let version_manifest = isahc::get(VERSION_MANIFEST_URL)?.json::<VersionManifest>()?;

    Ok(version_manifest.versions)
}

/// Returns all installed versions
pub fn get_installed_versions(minecraft_directory: &Path) -> Result<Vec<Version>> {
    let versions_dir = &minecraft_directory.join("versions");
    let dir_list = fs::read_dir(versions_dir)?;

    let mut version_list = vec![];
    for entry in dir_list {
        let path = entry?.path();
        let path = versions_dir.join(&path).join(&path).with_extension("json");

        if path.is_file() {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            let version_data = serde_json::from_reader(reader)?;
            version_list.push(version_data);
        }
    }

    Ok(version_list)
}

/// Returns all installed versions and all versions that Mojang offers to download
pub fn get_available_versions(minecraft_directory: &Path) -> Result<Vec<Version>> {
    let mut version_list = vec![];
    let mut version_check = vec![];

    for version in get_version_list()? {
        version_check.push(version.id.clone());
        version_list.push(version);
    }

    for version in get_installed_versions(minecraft_directory)? {
        if version_check
            .iter()
            .find(|&id| id.eq(&version.id))
            .is_none()
        {
            version_list.push(version);
        }
    }

    Ok(version_list)
}

/// Tries the find out the path to the default java executable
pub fn get_java_executable() -> String {
    todo!()
}

#[derive(Deserialize)]
pub struct Tile {
    pub sub_header: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct Article {
    pub default_tile: Tile,
    pub article_url: String,
    pub publish_date: String,
}

/// https://www.minecraft.net/content/minecraft-net/_jcr_content.articles.grid
/// TODO images
#[derive(Deserialize)]
pub struct Articles {
    pub article_grid: Vec<Article>,
    pub article_count: usize,
}

/// Checks if the given version exists
pub fn is_version_valid(id: &str, minecraft_directory: &Path) -> Result<bool> {
    if minecraft_directory.join("versions").join(id).is_dir() {
        return Ok(true);
    }

    let version_manifest = isahc::get(VERSION_MANIFEST_URL)?.json::<VersionManifest>()?;
    for version in version_manifest.versions {
        if version.id == id {
            return Ok(true);
        }
    }

    Ok(false)
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

    let mut resp = Request::get(url.to_string())
        .header("user-agent", "minecraft-launcher-lib-rs")
        .body(())?
        .send()
        .expect("Failed getting articles.grid");

    let articles = resp
        .json::<Articles>()
        .expect("Failed parsing articles.grid");

    Ok(articles)
}
