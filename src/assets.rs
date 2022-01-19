use crate::launchermeta::AssetIndex;
use crate::{download_file, BASE_DIR};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::{
    fs,
    path::{Path, PathBuf},
};
use url::Url;

#[derive(Deserialize)]
struct Assets {
    objects: HashMap<String, Object>,
}

#[derive(Deserialize, Clone)]
struct Object {
    hash: String,
}

lazy_static! {
    pub static ref ASSETS_DIR: PathBuf = BASE_DIR.join("assets");
    static ref OBJECTS_DIR: PathBuf = ASSETS_DIR.join("objects");
    static ref INDEXES_DIR: PathBuf = ASSETS_DIR.join("indexes");
}

fn download_asset(hash: &str) -> Result<(), Box<dyn Error>> {
    let first2 = &hash[..2];

    let path = OBJECTS_DIR.join(&first2).join(&hash);
    let url = Url::parse("https://resources.download.minecraft.net/")?
        .join(first2)?
        .join(hash)?;

    download_file(&url, &path)?;

    Ok(())
}

fn get_asset_index_path(id: &str) -> Result<PathBuf, Box<dyn Error>> {
    let index_path = INDEXES_DIR.join(id).with_extension("json");

    Ok(index_path)
}

fn read_asset_index(asset_index: &AssetIndex) -> Result<Vec<Object>, Box<dyn Error>> {
    let path = get_asset_index_path(&asset_index.id)?;

    if !Path::is_file(&path) {
        download_file(&asset_index.url, &path)?;
    }

    let data = fs::read_to_string(&path)?;
    let assets: Assets = serde_json::from_str(&data)?;
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
