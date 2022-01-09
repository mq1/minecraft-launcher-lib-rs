use crate::{BASE_DIR, download_file};
use serde::{Deserialize, Serialize};
use url::Url;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    pub r#type: String,
    pub url: Url,
}

#[derive(Deserialize)]
struct VersionManifest {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct URLObject {
    pub url: Url,
}

#[derive(Deserialize)]
pub struct Downloads {
    pub client: URLObject,
}

#[derive(Clone, Deserialize)]
pub struct Artifact {
    pub path: String,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct LibDownloads {
    pub artifact: Artifact,
    pub classifiers: Option<HashMap<String, Artifact>>,
}

#[derive(Deserialize)]
pub struct Os {
    pub name: String,
}

#[derive(Deserialize)]
pub struct Rule {
    pub action: String,
    pub os: Option<Os>,
}

#[derive(Deserialize)]
pub struct Library {
    pub downloads: LibDownloads,
    pub name: String,
    pub natives: Option<HashMap<String, String>>,
    pub rules: Option<Vec<Rule>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MinecraftMeta {
    pub id: String,
    pub asset_index: AssetIndex,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
}

lazy_static! {
    static ref MINECRAFT_MANIFESTS_DIR: PathBuf = BASE_DIR.join("meta").join("net.minecraft");
}

fn get_minecraft_manifest_path(minecraft_version: &str) -> PathBuf {
    let minecraft_version_manifest_path = MINECRAFT_MANIFESTS_DIR
        .join(minecraft_version)
        .with_extension("json");

    minecraft_version_manifest_path
}

pub fn get_minecraft_versions() -> Result<Vec<Version>, ureq::Error> {
    let resp: VersionManifest =
        ureq::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .call()?
            .into_json()?;

    Ok(resp.versions)
}

pub fn download_minecraft_manifest(
    minecraft_version: &str,
    minecraft_version_manifest_url: &Url,
) -> Result<(), Box<dyn Error>> {
    let minecraft_version_manifest_path = get_minecraft_manifest_path(minecraft_version);

    if !minecraft_version_manifest_path.is_file() {
        download_file(
            minecraft_version_manifest_url,
            &minecraft_version_manifest_path,
        )?;
    }

    Ok(())
}

pub fn read_minecraft_manifest(minecraft_version: &str) -> Result<MinecraftMeta, Box<dyn Error>> {
    let minecraft_version_manifest_path = get_minecraft_manifest_path(minecraft_version);
    let file = File::open(minecraft_version_manifest_path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}
