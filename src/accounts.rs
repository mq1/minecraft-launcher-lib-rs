use crate::{msa::MsaAccount, profile::get_user_profile, BASE_DIR};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    format_version: String,
    accounts: HashMap<String, MsaAccount>,
}

lazy_static! {
    static ref ACCOUNTS_PATH: PathBuf = BASE_DIR.join("accounts").with_extension("json");
}

fn get_new_config() -> Config {
    Config {
        format_version: "0".to_string(),
        accounts: HashMap::new(),
    }
}

fn write(config: &Config) -> Result<()> {
    let config = serde_json::to_string_pretty(config)?;
    fs::write(ACCOUNTS_PATH.as_path(), config)?;

    Ok(())
}

fn new() -> Result<Config> {
    let config = get_new_config();
    write(&config)?;

    Ok(config)
}

fn read() -> Result<Config> {
    if !Path::is_file(ACCOUNTS_PATH.as_path()) {
        return new();
    }

    let data = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let config = serde_json::from_str(&data)?;

    Ok(config)
}

fn add(account: MsaAccount) -> Result<()> {
    let mut config = read()?;
    let profile = get_user_profile(&account)?;
    config.accounts.insert(profile.id, account);

    write(&config)?;

    Ok(())
}

fn remove(id: &str) -> Result<()> {
    let mut config = read()?;
    config.accounts.remove(id);

    write(&config)?;

    Ok(())
}
