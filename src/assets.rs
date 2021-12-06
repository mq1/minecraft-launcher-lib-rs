use crate::util::{download_file, get_base_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Assets {
    objects: HashMap<String, Object>,
}

#[derive(Deserialize, Clone)]
struct Object {
    hash: String,
}

fn get_objects_dir() -> Result<PathBuf, Box<dyn Error>> {
    let path = get_base_dir()?
        .join("assets")
        .join("objects");

    Ok(path)
}

fn download_asset(hash: &str) -> Result<(), Box<dyn Error>> {
    let first2 = &hash[..2];

    let path = get_objects_dir()?
        .join(&first2)
        .join(&hash);

    let url = format!(
        "https://resources.download.minecraft.net/{}/{}",
        first2, hash
    );

    download_file(&url, &path)?;

    Ok(())
}

pub fn download_assets(asset_index_url: &str) -> Result<(), Box<dyn Error>> {
    let resp: Assets = ureq::get(&asset_index_url).call()?.into_json()?;

    let objects = resp.objects.values().collect::<Vec<&Object>>();
    for object in objects.iter() {
        download_asset(&object.hash)?;
    }

    Ok(())
}
