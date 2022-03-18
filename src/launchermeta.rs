use crate::{download_file, BASE_DIR};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use url::Url;

const VERSION_MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

#[derive(Deserialize)]
#[serde(untagged)]
enum ArgumentValue {
    One(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Argument {
    Simple(String),
    Explicit {
        rules: Vec<Rule>,
        value: ArgumentValue,
    },
}

#[derive(Deserialize)]
struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

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
pub struct AssetIndexMeta {
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
    pub arguments: Arguments,
    pub id: String,
    pub asset_index: AssetIndexMeta,
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

pub fn get_minecraft_versions() -> Result<Vec<Version>> {
    let resp: VersionManifest = ureq::get(VERSION_MANIFEST_URL).call()?.into_json()?;

    Ok(resp.versions)
}

pub fn download_minecraft_manifest(
    minecraft_version: &str,
    minecraft_version_manifest_url: &Url,
) -> Result<()> {
    let minecraft_version_manifest_path = get_minecraft_manifest_path(minecraft_version);

    // check if it was already downloaded
    if !minecraft_version_manifest_path.is_file() {
        download_file(
            minecraft_version_manifest_url,
            &minecraft_version_manifest_path,
        )?;
    }

    Ok(())
}

pub fn read_minecraft_manifest(minecraft_version: &str) -> Result<MinecraftMeta> {
    let minecraft_version_manifest_path = get_minecraft_manifest_path(minecraft_version);
    let file = File::open(minecraft_version_manifest_path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}

// TODO refactor these two methods

// TODO parse rules
pub fn get_jvm_args(minecraft_meta: &MinecraftMeta) -> Vec<String> {
    let mut final_args = Vec::new();

    for arg in &minecraft_meta.arguments.jvm {
        match arg {
            Argument::Simple(argument) => {
                final_args.push(argument.to_owned());
            }
            Argument::Explicit { rules, value } => match value {
                crate::launchermeta::ArgumentValue::One(argument) => {
                    final_args.push(argument.to_owned());
                }
                crate::launchermeta::ArgumentValue::Multiple(arguments) => {
                    final_args.append(&mut arguments.to_owned());
                }
            },
        }
    }

    final_args
}

// TODO parse rules
pub fn get_game_args(minecraft_meta: &MinecraftMeta) -> Vec<String> {
    let mut final_args = Vec::new();

    for arg in &minecraft_meta.arguments.game {
        match arg {
            Argument::Simple(argument) => {
                final_args.push(argument.to_owned());
            }
            Argument::Explicit { rules, value } => match value {
                crate::launchermeta::ArgumentValue::One(argument) => {
                    final_args.push(argument.to_owned());
                }
                crate::launchermeta::ArgumentValue::Multiple(arguments) => {
                    final_args.append(&mut arguments.to_owned());
                }
            },
        }
    }

    final_args
}
