use crate::assets::download_assets;
use crate::launchermeta::download_minecraft_manifest;
use crate::launchermeta::read_minecraft_manifest;
use crate::util::get_base_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, read_dir, File};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Config {
    minecraft_version: String,
}

fn get_instance_path<S: AsRef<str>>(name: S) -> Result<PathBuf, Box<dyn Error>> {
    let path = get_base_dir()?.join("instances").join(name.as_ref());

    Ok(path)
}

fn read_config<S: AsRef<str>>(instance_name: S) -> Result<Config, Box<dyn Error>> {
    let path = get_instance_path(instance_name)?.join("config.json");
    let config_file = File::open(path)?;
    let config = serde_json::from_reader(config_file)?;

    Ok(config)
}

fn write_config<S: AsRef<str>>(instance_name: S, config: Config) -> Result<(), Box<dyn Error>> {
    let path = get_instance_path(instance_name)?.join("config.json");
    let config_file = File::create(path)?;
    serde_json::to_writer(config_file, &config)?;

    Ok(())
}

pub fn get_instance_list() -> Result<Vec<String>, Box<dyn Error>> {
    let dir = get_base_dir()?.join("instances");

    let instance_list = read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .filter_map(|entry| entry.into_string().ok())
        .filter(|entry| entry.ne(".DS_Store"))
        .collect::<Vec<String>>();

    Ok(instance_list)
}

pub fn new_instance<S: AsRef<str>>(
    name: S,
    minecraft_version: S,
    minecraft_version_manifest_url: S,
) -> Result<(), Box<dyn Error>> {
    let instance_dir = get_instance_path(name.as_ref())?;
    create_dir_all(&instance_dir)?;

    let config = Config {
        minecraft_version: String::from(minecraft_version.as_ref()),
    };
    write_config(name.as_ref(), config)?;

    download_minecraft_manifest(
        minecraft_version.as_ref(),
        minecraft_version_manifest_url.as_ref(),
    )?;

    Ok(())
}

pub fn run_instance<S: AsRef<str>>(name: S) -> Result<(), Box<dyn Error>> {
    let config = read_config(name.as_ref())?;
    let minecraft_meta = read_minecraft_manifest(config.minecraft_version)?;

    download_assets(minecraft_meta.asset_index.url)?;

    Ok(())
}
