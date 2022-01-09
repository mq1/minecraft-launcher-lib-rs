use crate::{
    launchermeta::{Artifact, Library, MinecraftMeta},
    util::download_file,
    BASE_DIR,
};
use std::{error::Error, path::PathBuf};

lazy_static! {
    static ref LIBRARIES_DIR: PathBuf = BASE_DIR.join("libraries");
    static ref MINECRAFT_CLIENTS_DIR: PathBuf =
        LIBRARIES_DIR.join("com").join("mojang").join("minecraft");
    static ref OS: String = std::env::consts::OS.replace("macos", "osx");
}

fn download_client_jar(minecraft_meta: &MinecraftMeta) -> Result<(), Box<dyn Error>> {
    let path = MINECRAFT_CLIENTS_DIR
        .join(&minecraft_meta.id)
        .join(format!("minecraft-{}-client", &minecraft_meta.id))
        .with_extension("jar");

    download_file(&minecraft_meta.downloads.client.url, &path)?;

    Ok(())
}

fn download_artifact(artifact: &Artifact) -> Result<(), Box<dyn Error>> {
    let path = LIBRARIES_DIR.join(&artifact.path);
    download_file(&artifact.url, &path)?;

    Ok(())
}

// lazy/hacky implementation
// it kinda works but is's not flexible
fn is_valid_lib(lib: &&Library) -> bool {
    if lib.rules.is_none() {
        return true;
    }

    let rules = lib.rules.as_ref().unwrap();

    (rules.len() == 1 && OS.eq("osx")) || (rules.len() == 2 && OS.ne("osx"))
}

fn get_valid_libs(minecraft_meta: &MinecraftMeta) -> Vec<&Library> {
    minecraft_meta
        .libraries
        .iter()
        .filter(is_valid_lib)
        .collect()
}

fn get_native_artifact(lib: &&Library) -> Option<Artifact> {
    if lib.natives.is_some() {
        let natives = lib.natives.as_ref().unwrap();
        if natives.contains_key(OS.as_str()) {
            let artifact = lib.downloads.classifiers.as_ref().unwrap()[&natives[OS.as_str()]].clone();
            return Some(artifact);
        }
    }

    None
}

fn get_artifacts(libs: &Vec<&Library>) -> Vec<Artifact> {
    libs.iter()
        .map(|lib| lib.downloads.artifact.clone())
        .collect()
}

fn get_native_artifacts(libs: &Vec<&Library>) -> Vec<Artifact> {
    libs.iter()
        .map(get_native_artifact)
        .filter_map(|a| a)
        .collect()
}

pub fn download_libraries(
    minecraft_meta: &MinecraftMeta,
) -> Result<(Vec<Artifact>, Vec<Artifact>), Box<dyn Error>> {
    download_client_jar(minecraft_meta)?;

    let libs = get_valid_libs(minecraft_meta);
    let artifacts = get_artifacts(&libs);
    let native_artifacts = get_native_artifacts(&libs);

    for artifact in &artifacts {
        download_artifact(artifact)?;
    }

    for artifact in &native_artifacts {
        download_artifact(artifact)?;
    }

    Ok((artifacts, native_artifacts))
}
