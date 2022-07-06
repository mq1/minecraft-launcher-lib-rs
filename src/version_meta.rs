use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct VersionMeta {
    pub arguments: Arguments,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub assets: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i64,
    pub downloads: WelcomeDownloads,
    pub id: String,
    #[serde(rename = "javaVersion")]
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    pub logging: Logging,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "minimumLauncherVersion")]
    pub minimum_launcher_version: i64,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(rename = "type")]
    pub welcome_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Arguments {
    pub game: Vec<GameElement>,
    pub jvm: Vec<JvmElement>,
}

#[derive(Serialize, Deserialize)]
pub struct GameClass {
    pub rules: Vec<GameRule>,
    pub value: Value,
}

#[derive(Serialize, Deserialize)]
pub struct GameRule {
    pub action: Action,
    pub features: Features,
}

#[derive(Serialize, Deserialize)]
pub struct Features {
    pub is_demo_user: Option<bool>,
    pub has_custom_resolution: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct JvmClass {
    pub rules: Vec<JvmRule>,
    pub value: Value,
}

#[derive(Serialize, Deserialize)]
pub struct JvmRule {
    pub action: Action,
    pub os: PurpleOs,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleOs {
    pub name: Option<Name>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i64,
    #[serde(rename = "totalSize")]
    pub total_size: Option<i64>,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct WelcomeDownloads {
    pub client: ClientMappingsClass,
    pub client_mappings: ClientMappingsClass,
    pub server: ClientMappingsClass,
    pub server_mappings: ClientMappingsClass,
}

#[derive(Serialize, Deserialize)]
pub struct ClientMappingsClass {
    pub sha1: String,
    pub size: i64,
    pub url: String,
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JavaVersion {
    pub component: String,
    #[serde(rename = "majorVersion")]
    pub major_version: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
    pub name: String,
    pub rules: Option<Vec<LibraryRule>>,
}

#[derive(Serialize, Deserialize)]
pub struct LibraryDownloads {
    pub artifact: ClientMappingsClass,
}

#[derive(Serialize, Deserialize)]
pub struct LibraryRule {
    pub action: Action,
    pub os: Os,
}

#[derive(Serialize, Deserialize)]
pub struct Os {
    pub name: Name,
}

#[derive(Serialize, Deserialize)]
pub struct Logging {
    pub client: LoggingClient,
}

#[derive(Serialize, Deserialize)]
pub struct LoggingClient {
    pub argument: String,
    pub file: AssetIndex,
    #[serde(rename = "type")]
    pub client_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameElement {
    GameClass(GameClass),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    StringArray(Vec<String>),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum JvmElement {
    JvmClass(JvmClass),
    String(String),
}

#[derive(Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "allow")]
    Allow,
}

#[derive(Serialize, Deserialize)]
pub enum Name {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "osx")]
    Osx,
    #[serde(rename = "windows")]
    Windows,
}

pub fn get_version_meta(url: &Url) -> Result<VersionMeta> {
    let version_meta = attohttpc::get(url).send()?.json()?;

    Ok(version_meta)
}
