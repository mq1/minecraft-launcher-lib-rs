use std::{error::Error, path::PathBuf};

use crate::{util::{download_file, get_base_dir}, launchermeta::MinecraftMeta};

fn get_lib_dir() -> Result<PathBuf, Box<dyn Error>> {
    let path = get_base_dir()?.join("libraries");

    Ok(path)
}

fn download_client_jar(minecraft_meta: &MinecraftMeta) -> Result<(), Box<dyn Error>> {
    let path = get_lib_dir()?
        .join("com")
        .join("mojang")
        .join("minecraft")
        .join(&minecraft_meta.id)
        .join(format!("minecraft-{}-client.jar", &minecraft_meta.id));

    download_file(&minecraft_meta.downloads.client.url, &path)?;

    Ok(())
}

pub fn download_libraries(minecraft_meta: &MinecraftMeta) -> Result<(), Box<dyn Error>> {
    download_client_jar(minecraft_meta)?;

    // TODO download libraries

    Ok(())
}
