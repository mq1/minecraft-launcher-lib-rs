use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    msa::MsAccount,
    profile::{get_minecraft_account, get_user_profile, McAccount},
    BASE_DIR,
};

#[derive(Serialize, Deserialize)]
struct Account {
    id: String,
    msa: MsAccount,
    mca: McAccount,
}

#[derive(Serialize, Deserialize)]
struct Config {
    format_version: String,
    accounts: HashMap<String, Account>,
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
    let file = File::create(ACCOUNTS_PATH.as_path())?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, config)?;

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

    let file = File::open(ACCOUNTS_PATH.as_path())?;
    let reader = BufReader::new(file);

    let config = serde_json::from_reader(reader)?;

    Ok(config)
}

fn add(msa: MsAccount) -> Result<()> {
    let mca = get_minecraft_account(&msa.access_token)?;
    let profile = get_user_profile(&mca)?;

    let mut config = read()?;
    let account = Account {
        id: profile.id,
        msa,
        mca,
    };
    config.accounts.insert(profile.name, account);
    write(&config)?;

    Ok(())
}

fn remove(id: &str) -> Result<()> {
    let mut config = read()?;
    config.accounts.remove(id);

    write(&config)?;

    Ok(())
}
