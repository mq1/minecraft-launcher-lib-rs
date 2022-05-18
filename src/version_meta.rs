use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Allow,
    Disallow
}

#[derive(Deserialize)]
pub struct Os {
    pub name: Option<String>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

#[derive(Deserialize)]
pub struct Rule {
    pub action: Action,
    pub features: Option<HashMap<String, bool>>,
    pub os: Option<Os>,
}

impl Rule {
    pub fn parse(&self, options: HashMap<String, bool>) -> bool {
        let return_value = if self.action == Action::Allow { false } else { true };

        if self.os.is_some() {
            let os = self.os.as_ref().unwrap();
            
            if os.name.is_some() {
                let name = os.name.as_ref().unwrap().replace("osx", "macos");

                if name.as_str() != std::env::consts::OS {
                    return return_value;
                }
            }

            if os.arch.is_some() {
                let arch = os.arch.as_ref().unwrap();

                if arch.as_str() != std::env::consts::ARCH {
                    return return_value;
                }
            }

            if os.version.is_some() {
                let version = os.version.as_ref().unwrap();
                let re = Regex::new(version).unwrap();

                let actual_version = match os_info::get().version() {
                    os_info::Version::Unknown => "".to_string(),
                    os_info::Version::Semantic(major, minor, _) => format!("{major}.{minor}"),
                    os_info::Version::Rolling(_) => "".to_string(),
                    os_info::Version::Custom(_) => "".to_string(),
                };

                if !re.is_match(&actual_version) {
                    return return_value;
                }
            }
        }

        if self.features.is_some() {
            let features = self.features.as_ref().unwrap();

            for (key, _) in features {
                if key == "has_custom_resolution" && *options.get("customResolution").unwrap() {
                    return return_value;
                }
                if key == "is_demo_user" && *options.get("demo").unwrap() {
                    return return_value;
                }
            }
        }

        return_value
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    One(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Simple(String),
    Explicit {
        rules: Vec<Rule>,
        value: ArgumentValue,
    },
}

#[derive(Deserialize)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
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
    pub classifiers: Option<HashMap<String, Artifact>>
}

#[derive(Deserialize)]
pub struct Natives {
    pub linux: Option<String>,
    pub osx: Option<String>,
    pub windows: Option<String>
}

#[derive(Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub natives: Option<Natives>,
    pub rules: Option<Vec<Rule>>
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
    let version_meta = attohttpc::get(url).send()?.json()?;

    Ok(version_meta)
}
