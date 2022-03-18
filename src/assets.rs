use crate::{download_file, launchermeta::AssetIndexMeta, BASE_DIR};
use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

const RESOURCES_URL: &str = "https://resources.download.minecraft.net/";

#[derive(Deserialize)]
pub struct AssetIndex {
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

fn download_asset(hash: &str) -> Result<()> {
    let first2 = &hash[..2];

    let path = OBJECTS_DIR.join(&first2).join(&hash);
    let url = Url::parse(RESOURCES_URL)?.join(first2)?.join(hash)?;

    download_file(&url, &path)?;

    Ok(())
}

fn download_asset_index(asset_index_meta: &AssetIndexMeta) -> Result<()> {
    let path = INDEXES_DIR.join(asset_index_meta.id).with_extension("json");
    download_file(&asset_index_meta.url, &path)?;

    Ok(())
}

fn read_asset_index(id: &str) -> Result<AssetIndex> {
    let path = INDEXES_DIR.join(id).with_extension("json");

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let asset_index: AssetIndex = serde_json::from_reader(reader)?;

    Ok(asset_index)
}

pub fn download_assets(asset_index_meta: &AssetIndexMeta) -> Result<()> {
    download_asset_index(asset_index_meta)?;
    let asset_index = read_asset_index(&asset_index_meta.id)?;

    for (_, object) in asset_index.objects.into_iter() {
        download_asset(&object.hash)?;
    }

    Ok(())
}
