use std::process::Command;

use anyhow::Result;

use crate::{
    assets::{self, ASSETS_DIR},
    config, instances, launchermeta, libraries,
    profile::UserProfile,
};

fn update_last_launched(instance_name: &str) -> Result<()> {
    let mut config = config::read()?;
    config.last_launched_instance = instance_name.to_string();
    config::write(&config)?;

    Ok(())
}

pub fn launch(instance_name: &str, mc_profile: &UserProfile, mc_access_token: &str) -> Result<()> {
    update_last_launched(instance_name)?;

    let java_path = config::read()?.java.path;

    let config = instances::read_config(instance_name)?;
    let minecraft_meta = launchermeta::read_minecraft_manifest(&config.minecraft_version)?;

    assets::download_assets(&minecraft_meta.asset_index)?;
    let (artifacts, native_artifacts) = libraries::download_libraries(&minecraft_meta)?;
    let natives_dir = libraries::extract_natives(&native_artifacts)?;

    // parse jvm args
    let mut jvm_args: Vec<&str> = Vec::new();
    for arg in launchermeta::get_jvm_args(&minecraft_meta) {
        let final_arg = match arg.as_str() {
            "-Djava.library.path=${natives_directory}" => &format!(
                "-Djava.library.path={}",
                natives_dir.as_path().to_str().unwrap()
            ),
            "-Dminecraft.launcher.brand=${launcher_name}" => {
                &format!("-Dminecraft.launcher.brand={}", env!("CARGO_PKG_NAME"))
            }
            "-Dminecraft.launcher.version=${launcher_version}" => {
                &format!("-Dminecraft.launcher.version={}", env!("CARGO_PKG_VERSION"))
            }
            "${classpath}" => &get_classpath(&minecraft_meta),
            _ => &arg,
        };
        jvm_args.push(final_arg);
    }

    // parse game args
    let mut game_args: Vec<&str> = Vec::new();
    for arg in launchermeta::get_game_args(&minecraft_meta) {
        let final_arg = match arg.as_str() {
            "${auth_player_name}" => &mc_profile.name,
            "${version_name}" => &config.minecraft_version,
            "${game_directory}" => ".",
            "${assets_root}" => ASSETS_DIR.as_path().to_str().unwrap(),
            "${assets_index_name}" => &minecraft_meta.asset_index.id,
            "${auth_uuid}" => &mc_profile.id,
            "${auth_access_token}" => &mc_access_token,
            "${clientid}" => &format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            "${user_type}" => "mojang",
            "${version_type}" => &config.version_type,
            _ => &arg,
        };
        game_args.push(final_arg);
    }

    let mut final_args = Vec::new();
    final_args.append(&mut jvm_args);
    final_args.push(&config.main_class);
    final_args.append(&mut game_args);

    Command::new(java_path)
        .args(final_args)
        .current_dir(instances::get_instance_path(instance_name)?)
        .spawn()?;

    Ok(())
}
