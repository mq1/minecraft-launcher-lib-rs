use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::util::get_base_dir;

#[derive(Serialize, Deserialize)]
pub struct Account {
    name: String,
    id: String,
    access_token: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    accounts: Vec<Account>,
}

fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let path = get_base_dir()?.join("accounts.toml");

    Ok(path)
}

fn get_new_config() -> Config {
    Config { accounts: vec![] }
}

fn write(config: &Config) -> Result<(), Box<dyn Error>> {
    let path = get_config_path()?;
    let config = toml::to_string(config)?;
    fs::write(path, config)?;

    Ok(())
}

fn new() -> Result<Config, Box<dyn Error>> {
    let config = get_new_config();

    write(&config)?;

    Ok(config)
}

fn read() -> Result<Config, Box<dyn Error>> {
    let path = get_config_path()?;

    if !Path::is_file(&path) {
        return new();
    }

    let data = fs::read_to_string(&path)?;
    let config = toml::from_str(&data)?;

    Ok(config)
}

pub fn list() -> Result<Vec<Account>, Box<dyn Error>> {
    let config = read()?;

    Ok(config.accounts)
}

pub fn add(account: Account) -> Result<(), Box<dyn Error>> {
    let mut config = read()?;
    config.accounts.push(account);

    write(&config)?;

    Ok(())
}
