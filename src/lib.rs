use std::{
    fs::{self, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
};

use anyhow::Result;
use directories::ProjectDirs;
use url::Url;

pub mod accounts;
pub mod assets;
pub mod config;
pub mod instances;
pub mod launchermeta;
pub mod libraries;
mod msa;
pub mod profile;
pub mod launch;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate anyhow;

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

pub fn download_file(url: &Url, path: &Path) -> Result<()> {
    if path.exists() {
        println!("{:?} already present", path);
        return Ok(());
    }

    let dir = path.parent().ok_or(anyhow!("error getting parent dir"))?;
    fs::create_dir_all(dir)?;

    let mut resp = ureq::get(url.as_str()).call()?.into_reader();
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    io::copy(&mut resp, &mut writer)?;

    println!("downloaded file {} to {:?}", url, path);

    Ok(())
}
