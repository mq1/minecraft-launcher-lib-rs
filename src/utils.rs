use std::path::PathBuf;

use anyhow::Result;
use isahc::ReadResponseExt;
use serde::Deserialize;

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
struct VersionManifest {
    latest: LatestVersion
}

/// Returns the latest version of Minecraft
pub fn get_latest_version() -> Result<LatestVersion> {
    let resp = isahc::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")?.json::<VersionManifest>()?;

    Ok(resp.latest)
}
