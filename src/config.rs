use crate::util::get_base_dir;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::{Path, PathBuf}};

#[derive(Serialize, Deserialize)]
struct JavaConfig {
    path: String,
    memory: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    locale: String,
    java: JavaConfig,
}

impl AsRef<Config> for Config {
    fn as_ref(&self) -> &Config {
        self
    }
}

fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let path = get_base_dir()?.join("config.json");

    Ok(path)
}

pub fn get_default_config() -> Config {
    Config {
        locale: "en".to_string(),
        java: JavaConfig {
            path: "java".to_string(),
            memory: "2G".to_string(),
        },
    }
}

fn new() -> Result<Config, Box<dyn Error>> {
    let config = get_default_config();

    write(&config)?;

    Ok(config)
}

pub fn read() -> Result<Config, Box<dyn Error>> {
    let path = get_config_path()?;

    if Path::is_file(&path) {
        return new();
    }

    let file = File::open(path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}

pub fn write<C: AsRef<Config>>(config: C) -> Result<(), Box<dyn Error>> {
    let path = get_config_path()?;

    let file = File::open(path)?;
    serde_json::to_writer(file, config.as_ref())?;

    Ok(())
}
