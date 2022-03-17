use crate::{msa::Account, BASE_DIR};
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    accounts: Vec<Account>,
}

lazy_static! {
    static ref ACCOUNTS_PATH: PathBuf = BASE_DIR.join("accounts").with_extension("json");
}

fn get_new_config() -> Config {
    Config {
        accounts: Vec::new(),
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

fn add(account: Account) -> Result<()> {
    let mut config = read()?;
    config.accounts.push(account);

    write(&config)?;

    Ok(())
}

fn remove(account: &Account) -> Result<()> {
    let mut config = read()?;
    config.accounts = config
        .accounts
        .into_iter()
        .filter(|a| !a.access_token.eq(&account.access_token))
        .collect();

    write(&config)?;

    Ok(())
}
