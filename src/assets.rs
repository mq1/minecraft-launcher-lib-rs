use crate::launchermeta::AssetIndex;
use crate::util::{download_file, get_base_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
struct Assets {
    objects: HashMap<String, Object>,
}

#[derive(Deserialize, Clone)]
struct Object {
    hash: String,
}

fn get_assets_dir() -> Result<PathBuf, Box<dyn Error>> {
    let path = get_base_dir()?.join("assets");

    Ok(path)
}

fn download_asset(hash: &str) -> Result<(), Box<dyn Error>> {
    let first2 = &hash[..2];

    let path = get_assets_dir()?.join("objects").join(&first2).join(&hash);

    let url = format!(
        "https://resources.download.minecraft.net/{}/{}",
        first2, hash
    );

    download_file(&url, &path)?;

    Ok(())
}

fn get_asset_index_path(id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let index_path = get_assets_dir()?
        .join("indexes")
        .join(format!("{}.json", id));

    Ok(index_path)
}

fn read_asset_index(asset_index: &AssetIndex) -> Result<Vec<Object>, Box<dyn Error>> {
    let path = get_asset_index_path(&asset_index.id)?;

    if !Path::is_file(&path) {
        download_file(&asset_index.url, &path)?;
    }

    let data = fs::read_to_string(&path)?;
    let assets: Assets = toml::from_str(&data)?;
    let objects = assets.objects.into_values().collect();

    Ok(objects)
}

pub fn download_assets(asset_index: &AssetIndex) -> Result<(), Box<dyn Error>> {
    let objects = read_asset_index(asset_index)?;

    for object in objects.iter() {
        download_asset(&object.hash)?;
    }

    Ok(())
}
