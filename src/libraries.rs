use crate::{
    download_file,
    launchermeta::{Artifact, Library, MinecraftMeta},
    BASE_DIR,
};
use std::{fs, io, path::PathBuf};

use anyhow::Result;

lazy_static! {
    static ref LIBRARIES_DIR: PathBuf = BASE_DIR.join("libraries");
    static ref NATIVES_TMP_DIR: PathBuf = BASE_DIR.join("natives-tmp");
    static ref MINECRAFT_CLIENTS_DIR: PathBuf =
        LIBRARIES_DIR.join("com").join("mojang").join("minecraft");
    static ref OS: String = std::env::consts::OS.replace("macos", "osx");
}

async fn download_client_jar(minecraft_meta: &MinecraftMeta) -> Result<()> {
    let path = MINECRAFT_CLIENTS_DIR
        .join(&minecraft_meta.id)
        .join(format!("minecraft-{}-client", &minecraft_meta.id))
        .with_extension("jar");

    download_file(&minecraft_meta.downloads.client.url, &path).await?;

    Ok(())
}

async fn download_artifact(artifact: &Artifact) -> Result<()> {
    let path = LIBRARIES_DIR.join(&artifact.path);
    download_file(&artifact.url, &path).await?;

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
            let artifact =
                lib.downloads.classifiers.as_ref().unwrap()[&natives[OS.as_str()]].clone();
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

pub fn extract_natives(native_artifacts: &Vec<Artifact>) -> Result<PathBuf> {
    fs::remove_dir_all(NATIVES_TMP_DIR.as_path())?;
    fs::create_dir_all(NATIVES_TMP_DIR.as_path())?;

    for artifact in native_artifacts {
        let path = LIBRARIES_DIR.join(&artifact.path);
        let jarfile = fs::File::open(path)?;

        let mut archive = zip::ZipArchive::new(jarfile)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            let outpath = match file.enclosed_name() {
                Some(path) => NATIVES_TMP_DIR.join(path),
                None => continue,
            };

            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
        }
    }

    Ok(NATIVES_TMP_DIR.as_path().into())
}

pub async fn download_libraries(
    minecraft_meta: &MinecraftMeta,
) -> Result<(Vec<Artifact>, Vec<Artifact>)> {
    download_client_jar(minecraft_meta).await?;

    let libs = get_valid_libs(minecraft_meta);
    let artifacts = get_artifacts(&libs);
    let native_artifacts = get_native_artifacts(&libs);

    for artifact in &artifacts {
        download_artifact(artifact).await?;
    }

    for artifact in &native_artifacts {
        download_artifact(artifact).await?;
    }

    Ok((artifacts, native_artifacts))
}
