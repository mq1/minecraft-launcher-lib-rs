use crate::util::{download_file, get_base_dir};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Version {
    pub id: String,
    pub r#type: String,
    pub url: String,
}

#[derive(Deserialize)]
struct VersionManifest {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftMeta {
    pub asset_index: AssetIndex,
}

fn get_minecraft_manifest_path<S: AsRef<str>>(
    minecraft_version: S,
) -> Result<PathBuf, Box<dyn Error>> {
    let file_name = format!("{}.json", minecraft_version.as_ref());

    let minecraft_version_manifest_path = get_base_dir()?
        .join("meta")
        .join("net.minecraft")
        .join(file_name);

    Ok(minecraft_version_manifest_path.to_path_buf())
}

pub fn get_minecraft_versions() -> Result<Vec<Version>, reqwest::Error> {
    let resp =
        reqwest::blocking::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")?
            .json::<VersionManifest>()?;

    Ok(resp.versions)
}

pub fn download_minecraft_manifest<S: AsRef<str>>(
    minecraft_version: S,
    minecraft_version_manifest_url: S,
) -> Result<(), Box<dyn Error>> {
    let minecraft_version_manifest_path = get_minecraft_manifest_path(minecraft_version)?;

    download_file(
        minecraft_version_manifest_url,
        minecraft_version_manifest_path,
    )?;

    Ok(())
}

pub fn read_minecraft_manifest<S: AsRef<str>>(
    minecraft_version: S,
) -> Result<MinecraftMeta, Box<dyn Error>> {
    let minecraft_version_manifest_path = get_minecraft_manifest_path(minecraft_version)?;
    let file = File::open(minecraft_version_manifest_path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}
