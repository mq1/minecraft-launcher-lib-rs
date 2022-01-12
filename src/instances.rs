use crate::assets::download_assets;
use crate::config;
use crate::launchermeta::download_minecraft_manifest;
use crate::launchermeta::read_minecraft_manifest;
use crate::libraries::download_libraries;
use crate::BASE_DIR;
use crate::libraries::extract_natives;
use serde::{Deserialize, Serialize};
use url::Url;
use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Config {
    minecraft_version: String,
}

lazy_static! {
    static ref INSTANCES_DIR: PathBuf = BASE_DIR.join("instances");
}

fn get_instance_path(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = INSTANCES_DIR.join(name);

    Ok(path)
}

fn get_config_path(instance_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = get_instance_path(instance_name)?
        .join("config")
        .with_extension("toml");

    Ok(path)
}

fn read_config(instance_name: &str) -> Result<Config, Box<dyn Error>> {
    let path = get_config_path(instance_name)?;
    let data = fs::read_to_string(path)?;
    let config = toml::from_str(&data)?;

    Ok(config)
}

fn write_config(instance_name: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let path = get_config_path(instance_name)?;
    let data = toml::to_string(config)?;
    fs::write(path, data)?;

    Ok(())
}

pub fn get_instance_list() -> Result<Vec<String>, Box<dyn Error>> {
    let instance_list = read_dir(INSTANCES_DIR.as_path())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .filter_map(|entry| entry.into_string().ok())
        .filter(|entry| entry.ne(".DS_Store"))
        .collect::<Vec<String>>();

    Ok(instance_list)
}

pub fn new_instance(
    name: &str,
    minecraft_version: &str,
    minecraft_version_manifest_url: &Url,
) -> Result<(), Box<dyn Error>> {
    let instance_dir = get_instance_path(name)?;
    create_dir_all(&instance_dir)?;

    let config = Config {
        minecraft_version: minecraft_version.to_owned(),
    };
    write_config(name, &config)?;

    download_minecraft_manifest(minecraft_version, minecraft_version_manifest_url)?;

    Ok(())
}

pub fn remove_instance(name: &str) -> Result<(), Box<dyn Error>> {
    let instance_dir = get_instance_path(name)?;

    fs::remove_dir_all(instance_dir)?;

    Ok(())
}

pub fn rename_instance(old_name: &str, new_name: &str) -> Result<(), Box<dyn Error>> {
    let old_instance_dir = get_instance_path(old_name)?;
    let new_instance_dir = old_instance_dir.parent().unwrap().join(new_name);

    fs::rename(old_instance_dir, new_instance_dir)?;

    Ok(())
}

pub fn run_instance(name: &str) -> Result<(), Box<dyn Error>> {
    // update last runned instance
    let global_config = config::read()?;
    let global_config = config::Config {
        last_runned_instance: name.to_owned(),
        ..global_config
    };
    config::write(&global_config)?;

    let config = read_config(name)?;
    let minecraft_meta = read_minecraft_manifest(&config.minecraft_version)?;

    download_assets(&minecraft_meta.asset_index)?;
    let (artifacts, native_artifacts) = download_libraries(&minecraft_meta)?;
    let natives_dir = extract_natives(&native_artifacts)?;

    Ok(())
}
