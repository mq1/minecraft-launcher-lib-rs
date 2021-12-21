use std::{error::Error, path::{PathBuf, Path}};

use crate::{util::{download_file, get_base_dir}, launchermeta::{MinecraftMeta, Artifact, Library}};

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
    let path = get_lib_dir()?
        .join(relative_path);

    download_file(&artifact.url, &path)?;

    Ok(())
}

fn is_os(os: &str) -> bool {
    let actual_os = std::env::consts::OS;
    let actual_os = actual_os.replace("macos", "osx");

    actual_os == String::from(os)
}

fn is_valid_lib(lib: &&Library) -> bool {
    if lib.rules.is_some() {
        for rule in lib.rules.as_ref().unwrap() {
            if rule.os.is_some() {
                if rule.action.eq("disallow") && is_os(&rule.os.as_ref().unwrap().name) {
                    return false;
                }
            }
        }
    }

    true
}

pub fn download_libraries(minecraft_meta: &MinecraftMeta) -> Result<(), Box<dyn Error>> {
    download_client_jar(minecraft_meta)?;

    let libs: Vec<&Library> = minecraft_meta.libraries.iter().filter(|lib| is_valid_lib(lib)).collect();
    for lib in libs {
        download_artifact(&lib.downloads.artifact)?;
    }

    Ok(())
}
