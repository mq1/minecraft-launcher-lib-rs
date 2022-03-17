use std::{
    fs,
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
pub mod msa;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

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

pub async fn download_file(url: &Url, path: &Path) -> Result<()> {
    if path.exists() {
        println!("{:?} already present", path);
        return Ok(());
    }

    let dir = path.parent().ok_or(anyhow!("error getting parent dir"))?;
    fs::create_dir_all(dir)?;

    let contents = surf::get(url.as_str())
        .recv_bytes()
        .await
        .map_err(|e| anyhow!(e))?;

    fs::write(path, contents)?;

    println!("downloaded file {} to {:?}", url, path);

    Ok(())
}
