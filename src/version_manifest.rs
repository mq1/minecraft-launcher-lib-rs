use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Deserialize)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize)]
pub struct Version {
    pub id: String,
    pub r#type: String,
    pub url: Url,
    pub time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersion,
    pub versions: Vec<Version>,
}

pub fn get_version_manifest() -> Result<VersionManifest> {
    let version_manifest = attohttpc::get(VERSION_MANIFEST_URL).send()?.json()?;

    Ok(version_manifest)
}
