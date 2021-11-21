use crate::util::{download_file, get_base_dir};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize)]
struct Assets {
    objects: HashMap<String, Object>,
}

#[derive(Deserialize, Clone)]
struct Object {
    hash: String,
}

fn download_asset(hash: String) -> Result<(), Box<dyn Error>> {
    let first2 = &hash[..2];

    let path = get_base_dir()?
        .join("assets")
        .join("objects")
        .join(&first2)
        .join(&hash);

    let url = format!(
        "https://resources.download.minecraft.net/{}/{}",
        first2, hash
    );

    download_file(url, path)?;

    Ok(())
}

pub fn download_assets(asset_index_url: String) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get(asset_index_url)?.json::<Assets>()?;
    let objects = resp.objects.values().cloned().collect::<Vec<Object>>();
    for object in objects.into_iter() {
        download_asset(object.hash)?;
    }

    Ok(())
}
