use anyhow::Result;
use chrono::{DateTime, Utc};
use isahc::ReadResponseExt;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct Arguments {
    pub game: Vec<String>,
    pub jvm: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    pub total_size: usize,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct Download {
    pub sha1: String,
    pub size: usize,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct Downloads {
    pub client: Download,
    pub client_mappings: Download,
    pub server: Download,
    pub server_mappings: Download,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: usize,
}

#[derive(Deserialize)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: usize,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Artifact,
}

#[derive(Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
}

#[derive(Deserialize)]
pub struct File {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct Client {
    pub argument: String,
    pub file: File,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct Logging {
    pub client: Client,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionMeta {
    pub arguments: Arguments,
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: usize,
    pub downloads: Downloads,
    pub id: String,
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    pub logging: Logging,
    pub main_class: String,
    pub minimum_launcher_version: usize,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    pub r#type: String,
}

pub fn get_version_meta(url: &Url) -> Result<VersionMeta> {
    let version_meta = isahc::get(url.as_str())?.json()?;

    Ok(version_meta)
}
