use crate::util::get_base_dir;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct JavaConfig {
    path: String,
    memory: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub locale: String,
    pub java: JavaConfig,
    pub last_runned_instance: String,
}

lazy_static! {
    static ref CONFIG_PATH: PathBuf = get_base_dir().unwrap().join("config.toml");
}

pub fn get_default_config() -> Config {
    Config {
        locale: "en".to_string(),
        java: JavaConfig {
            path: "java".to_string(),
            memory: "2G".to_string(),
        },
        last_runned_instance: String::new(),
    }
}

pub fn write(config: &Config) -> Result<(), Box<dyn Error>> {
    let config = toml::to_string(config)?;
    fs::write(CONFIG_PATH.as_path(), config)?;

    Ok(())
}

pub fn new() -> Result<Config, Box<dyn Error>> {
    let config = get_default_config();

    write(&config)?;

    Ok(config)
}

pub fn read() -> Result<Config, Box<dyn Error>> {
    if !Path::is_file(CONFIG_PATH.as_path()) {
        return new();
    }

    let data = fs::read_to_string(CONFIG_PATH.as_path())?;
    let config = toml::from_str(&data)?;

    Ok(config)
}
