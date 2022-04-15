use std::{path::{Path, PathBuf}, fs};

use anyhow::Result;

/// creates a new instance and returns its path
pub fn new(name: &str, instances_dir: &Path) -> Result<PathBuf> {
    let path = instances_dir.join(name);
    fs::create_dir_all(&path)?;

    Ok(path)
}
