use crate::BASE_DIR;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
pub struct JavaConfig {
    pub path: String,
    pub memory: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub locale: String,
    pub java: JavaConfig,
    pub last_runned_instance: String,
}

lazy_static! {
    pub static ref CONFIG_PATH: PathBuf = BASE_DIR.join("config").with_extension("json");
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
    let config = serde_json::to_string_pretty(config)?;
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
    let config = serde_json::from_str(&data)?;

    Ok(config)
}
