use std::{path::{Path, PathBuf}, fs};

use anyhow::Result;

/// creates a new instance and returns its path
pub fn new(name: &str, instances_dir: &Path) -> Result<PathBuf> {
    let path = instances_dir.join(name);
    fs::create_dir_all(&path)?;

    Ok(path)
}

/// renames an instance and returns its path
pub fn rename(old_name: &str, new_name: &str, instances_dir: &Path) -> Result<PathBuf> {
    let old_path = instances_dir.join(old_name);
    let new_path = instances_dir.join(new_name);
    fs::rename(old_path, &new_path)?;

    Ok(new_path)
}

/// removes an instance
pub fn remove(name: &str, instances_dir: &Path) -> Result<()> {
    let path = instances_dir.join(name);
    fs::remove_dir_all(path)?;

    Ok(())
}
