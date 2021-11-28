use directories::ProjectDirs;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io;
use std::path::{Path, PathBuf};

pub fn get_base_dir() -> Result<PathBuf, Box<dyn Error>> {
    let dir = ProjectDirs::from("eu", "mq1", "runmc").ok_or("Error getting base directory")?;
    let dir = dir.data_dir().to_path_buf();

    Ok(dir)
}

pub fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        println!("{:?} already present", path);
        return Ok(());
    }

    let dir = path.parent().ok_or("error getting parent dir")?;
    create_dir_all(dir)?;

    let mut resp = ureq::get(url).call()?.into_reader();
    let mut out = File::create(path)?;
    io::copy(&mut resp, &mut out)?;

    println!("downloaded file {} to {:?}", url, path);

    Ok(())
}
