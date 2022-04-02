use std::path::PathBuf;

use anyhow::Result;
use isahc::ReadResponseExt;
use serde::Deserialize;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

/// Returns the default path to the .minecraft directory
pub fn get_minecraft_directory() -> Result<PathBuf> {
    let base_dirs = directories::BaseDirs::new().ok_or(anyhow!("BaseDirs not found"))?;

    Ok(match std::env::consts::OS {
        "windows" => base_dirs.data_dir().join(".minecraft"),
        "linux" => base_dirs.data_dir().join("minecraft"),
        _ => base_dirs.home_dir().join(".minecraft")
    })
}

#[derive(Deserialize)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String
}

#[derive(Deserialize)]
pub struct Version {
    id: String,
    r#type: String
}

#[derive(Deserialize)]
struct VersionManifest {
    latest: LatestVersion,
    versions: Vec<Version>
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
