use std::path::PathBuf;

use anyhow::Result;

/// Returns the default path to the .minecraft directory
pub fn get_minecraft_directory() -> Result<PathBuf> {
    let base_dirs = directories::BaseDirs::new().ok_or(anyhow!("BaseDirs not found"))?;

    Ok(match std::env::consts::OS {
        "windows" => base_dirs.data_dir().join(".minecraft"),
        "linux" => base_dirs.data_dir().join("minecraft"),
        _ => base_dirs.home_dir().join(".minecraft")
    })
}
