use std::path::PathBuf;

use directories::ProjectDirs;

pub mod accounts;
pub mod assets;
pub mod config;
pub mod instances;
pub mod launchermeta;
pub mod libraries;
pub mod util;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref BASE_DIR: PathBuf = ProjectDirs::from("eu", "mq1", "runmc").unwrap().data_dir().to_path_buf();
}
