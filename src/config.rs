use serde::{Deserialize, Serialize};
use std::{error::Error, fs::File, path::PathBuf};
use crate::util::get_base_dir;

#[derive(Serialize, Deserialize, Clone)]
struct JavaConfig {
    path: String,
    memory: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    locale: String,
    java: JavaConfig,
}

fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let path = get_base_dir()?.join("config.json");

    Ok(path)
}

fn getDefaultConfig() -> Config {
    Config {
        locale: "en".to_string(),
        java: JavaConfig {
            path: "java".to_string(),
            memory: "2G".to_string(),
        },
    }
}

fn read() -> Result<Config, Box<dyn Error>> {
    let path = get_config_path()?;

    let file = File::open(path)?;
    let config = serde_json::from_reader(file)?;

    Ok(config)
}

fn write(config: Config) -> Result<(), Box<dyn Error>> {
    let path = get_config_path()?;

    let file = File::open(path)?;
    serde_json::to_writer(file, &config)?;

    Ok(())
}

fn new() -> Result<Config, Box<dyn Error>> {
    let config = getDefaultConfig();

    write(config.clone())?;

    Ok(config)
}
