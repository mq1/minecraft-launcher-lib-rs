use directories::ProjectDirs;
use std::{
    error::Error,
    path::{Path, PathBuf}, fs::{self, File}, io,
};
use url::Url;

pub mod accounts;
pub mod assets;
pub mod config;
pub mod instances;
pub mod launchermeta;
pub mod libraries;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref BASE_DIR: PathBuf = {
        let dir = ProjectDirs::from("eu", "mq1", "runmc")
            .unwrap()
            .data_dir()
            .to_path_buf();

        fs::create_dir_all(&dir).unwrap();

        dir
    };
}

pub fn download_file(url: &Url, path: &Path) -> Result<(), Box<dyn Error>> {
    if path.exists() {
        println!("{:?} already present", path);
        return Ok(());
    }

    let dir = path.parent().ok_or("error getting parent dir")?;
    fs::create_dir_all(dir)?;

    let mut resp = ureq::get(url.as_str()).call()?.into_reader();
    let mut out = File::create(path)?;
    io::copy(&mut resp, &mut out)?;

    println!("downloaded file {} to {:?}", url, path);

    Ok(())
}
