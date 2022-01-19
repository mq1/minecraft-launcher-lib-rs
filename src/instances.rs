use crate::accounts::get_user_profile;
use crate::accounts::Account;
use crate::assets::download_assets;
use crate::assets::ASSETS_DIR;
use crate::config;
use crate::launchermeta::download_minecraft_manifest;
use crate::launchermeta::get_game_args;
use crate::launchermeta::get_jvm_args;
use crate::launchermeta::read_minecraft_manifest;
use crate::libraries::download_libraries;
use crate::libraries::extract_natives;
use crate::BASE_DIR;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;
use std::process::Command;
use url::Url;

#[derive(Serialize, Deserialize)]
struct Config {
    minecraft_version: String,
    version_type: String,
    main_class: String,
}

lazy_static! {
    static ref INSTANCES_DIR: PathBuf = BASE_DIR.join("instances");
}

fn get_instance_path(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = INSTANCES_DIR.join(name);

    Ok(path)
}

fn get_config_path(instance_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let path = get_instance_path(instance_name)?
        .join("config")
        .with_extension("json");

    Ok(path)
}

fn read_config(instance_name: &str) -> Result<Config, Box<dyn Error>> {
    let path = get_config_path(instance_name)?;
    let data = fs::read_to_string(path)?;
    let config = serde_json::from_str(&data)?;

    Ok(config)
}

fn write_config(instance_name: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let path = get_config_path(instance_name)?;
    let data = serde_json::to_string_pretty(config)?;
    fs::write(path, data)?;

    Ok(())
}

pub fn get_instance_list() -> Result<Vec<String>, Box<dyn Error>> {
    let instance_list = read_dir(INSTANCES_DIR.as_path())?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name())
        .filter_map(|entry| entry.into_string().ok())
        .filter(|entry| entry.ne(".DS_Store"))
        .collect::<Vec<String>>();

    Ok(instance_list)
}

pub fn new_instance(
    name: &str,
    minecraft_version: &str,
    minecraft_version_manifest_url: &Url,
) -> Result<(), Box<dyn Error>> {
    let instance_dir = get_instance_path(name)?;
    create_dir_all(&instance_dir)?;

    let config = Config {
        minecraft_version: minecraft_version.to_owned(),
        version_type: String::from("Vanilla"),
        main_class: String::from("net.minecraft.launchwrapper.Launch"),
    };
    write_config(name, &config)?;

    download_minecraft_manifest(minecraft_version, minecraft_version_manifest_url)?;

    Ok(())
}

pub fn remove_instance(name: &str) -> Result<(), Box<dyn Error>> {
    let instance_dir = get_instance_path(name)?;

    fs::remove_dir_all(instance_dir)?;

    Ok(())
}

pub fn rename_instance(old_name: &str, new_name: &str) -> Result<(), Box<dyn Error>> {
    let old_instance_dir = get_instance_path(old_name)?;
    let new_instance_dir = old_instance_dir.parent().unwrap().join(new_name);

    fs::rename(old_instance_dir, new_instance_dir)?;

    Ok(())
}

pub fn run_instance(name: &str, account: &Account) -> Result<(), Box<dyn Error>> {
    // update last runned instance
    let global_config = config::read()?;
    let global_config = config::Config {
        last_runned_instance: name.to_owned(),
        ..global_config
    };
    config::write(&global_config)?;

    let config = read_config(name)?;
    let minecraft_meta = read_minecraft_manifest(&config.minecraft_version)?;

    download_assets(&minecraft_meta.asset_index)?;
    let (artifacts, native_artifacts) = download_libraries(&minecraft_meta)?;
    let natives_dir = extract_natives(&native_artifacts)?;

    let user_profile = get_user_profile(account)?;

    // parse jvm args
    let mut jvm_args: Vec<&str> = Vec::new();
    for arg in get_jvm_args(&minecraft_meta) {
        let final_arg = match arg.as_str() {
            "-Djava.library.path=${natives_directory}" => &format!("-Djava.library.path={}", natives_dir.as_path().to_str().unwrap()),
            "-Dminecraft.launcher.brand=${launcher_name}" => &format!("-Dminecraft.launcher.brand={}", env!("CARGO_PKG_NAME")),
            "-Dminecraft.launcher.version=${launcher_version}" => &format!("-Dminecraft.launcher.version={}", env!("CARGO_PKG_VERSION")),
            "${classpath}" => &get_classpath(&minecraft_meta),
            _ => &arg
        };
        jvm_args.push(final_arg);
    }

    // parse game args
    let mut game_args: Vec<&str> = Vec::new();
    for arg in get_game_args(&minecraft_meta) {
        let final_arg = match arg.as_str() {
            "${auth_player_name}" => &user_profile.name,
            "${version_name}" => &config.minecraft_version,
            "${game_directory}" => ".",
            "${assets_root}" => ASSETS_DIR.as_path().to_str().unwrap(),
            "${assets_index_name}" => &minecraft_meta.asset_index.id,
            "${auth_uuid}" => &user_profile.id,
            "${auth_access_token}" => &account.access_token,
            "${clientid}" => &format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            "${user_type}" => "mojang",
            "${version_type}" => &config.version_type,
            _ => &arg
        };
        game_args.push(final_arg);
    }

    let mut final_args = Vec::new();
    final_args.append(&mut jvm_args);
    final_args.push(&config.main_class);
    final_args.append(&mut game_args);

    Command::new(global_config.java.path)
        .args(final_args)
        .current_dir(get_instance_path(name)?)
        .spawn()?;

    Ok(())
}
