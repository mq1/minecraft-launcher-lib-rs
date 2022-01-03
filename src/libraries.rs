use std::{
    error::Error,
    path::{Path, PathBuf},
};

use crate::{
    launchermeta::{Artifact, Library, MinecraftMeta},
    util::{download_file, get_base_dir},
};

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

fn download_artifact(artifact: &Artifact) -> Result<(), Box<dyn Error>> {
    let relative_path = Path::new(&artifact.path);
    let path = get_lib_dir()?.join(relative_path);

    download_file(&artifact.url, &path)?;

    Ok(())
}

fn get_os() -> String {
    let os = std::env::consts::OS;
    let os = os.replace("macos", "osx");

    os
}

// lazy/hacky implementation
// it kinda works but is's not flexible
fn is_valid_lib(lib: &&Library) -> bool {
    if lib.rules.is_none() {
        true
    }

    let rules = lib.rules.unwrap();
    let os = get_os();

    (rules.len() == 1 && os.eq("osx")) || (rules.len() == 2 && os.ne("osx"))
}

fn get_valid_libs(minecraft_meta: &MinecraftMeta) -> Vec<&Library> {
    minecraft_meta
        .libraries
        .iter()
        .filter(is_valid_lib)
        .collect()
}

pub fn download_libraries(minecraft_meta: &MinecraftMeta) -> Result<Vec<Library>, Box<dyn Error>> {
    download_client_jar(minecraft_meta)?;

    let os = get_os();

    let libs = get_valid_libs(minecraft_meta);

    for lib in libs {
        download_artifact(&lib.downloads.artifact)?;

        if lib.natives.is_some() {
            let natives = lib.natives.as_ref().unwrap();
            if natives.contains_key(&os) {
                let artifact = &lib.downloads.classifiers.as_ref().unwrap()[&natives[&os]];
                download_artifact(artifact)?;
            }
        }
    }

    Ok(libs)
}
