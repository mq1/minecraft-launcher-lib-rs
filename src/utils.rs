use std::{path::{PathBuf, Path}, fs::{self, File}, io::BufReader};

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
    pub id: String,
    pub r#type: String
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
