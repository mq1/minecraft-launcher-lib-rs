use crate::{launchermeta, BASE_DIR};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{self, create_dir_all, read_dir};
use std::path::PathBuf;
use url::Url;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub minecraft_version: String,
    pub version_type: String,
    pub main_class: String,
}

lazy_static! {
    static ref INSTANCES_DIR: PathBuf = BASE_DIR.join("instances");
}

pub fn get_instance_path(name: &str) -> Result<PathBuf> {
    let path = INSTANCES_DIR.join(name);

    Ok(path)
}

fn get_config_path(instance_name: &str) -> Result<PathBuf> {
    let path = get_instance_path(instance_name)?
        .join("config")
        .with_extension("json");

    Ok(path)
}

pub fn read_config(instance_name: &str) -> Result<Config> {
    let path = get_config_path(instance_name)?;
    let data = fs::read_to_string(path)?;
    let config = serde_json::from_str(&data)?;

    Ok(config)
}

fn write_config(instance_name: &str, config: &Config) -> Result<()> {
    let path = get_config_path(instance_name)?;
    let data = serde_json::to_string_pretty(config)?;
    fs::write(path, data)?;

    Ok(())
}

pub fn get_instance_list() -> Result<Vec<String>> {
    let instance_list = read_dir(INSTANCES_DIR.as_path())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .filter_map(|entry| entry.into_string().ok())
        .filter(|entry| entry.ne(".DS_Store"))
        .collect::<Vec<String>>();

    Ok(instance_list)
}

pub async fn new_instance(
    name: &str,
    minecraft_version: &str,
    minecraft_version_manifest_url: &Url,
) -> Result<()> {
    let instance_dir = get_instance_path(name)?;
    create_dir_all(&instance_dir)?;

    let config = Config {
        minecraft_version: minecraft_version.to_owned(),
        version_type: String::from("Vanilla"),
        main_class: String::from("net.minecraft.launchwrapper.Launch"),
    };
    write_config(name, &config)?;

    launchermeta::download_minecraft_manifest(minecraft_version, minecraft_version_manifest_url)?;

    Ok(())
}

pub fn remove_instance(name: &str) -> Result<()> {
    let instance_dir = get_instance_path(name)?;

    fs::remove_dir_all(instance_dir)?;

    Ok(())
}

pub fn rename_instance(old_name: &str, new_name: &str) -> Result<()> {
    let old_instance_dir = get_instance_path(old_name)?;
    let new_instance_dir = old_instance_dir.parent().unwrap().join(new_name);

    fs::rename(old_instance_dir, new_instance_dir)?;

    Ok(())
}
