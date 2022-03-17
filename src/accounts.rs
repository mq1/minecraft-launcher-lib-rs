use crate::{BASE_DIR, msa::Account};
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Config {
    accounts: Vec<Account>,
}

const CLIENT_ID: &str = "2000ea79-d993-4591-b9c4-e678f82ae1db";
const SCOPE: &str = "XboxLive.signin offline_access";

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

/// returns xbl_token
async fn authenticate_with_xbl(ms_access_token: &str) -> Result<String> {
    const AUTH_URL: &str = "https://user.auth.xboxlive.com/user/authenticate";

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Response {
        token: String,
    }

    let query = json!({
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": format!("d={ms_access_token}")
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
    });

    let resp: Response = surf::post(AUTH_URL)
        .body_json(&query)
        .map_err(|e| anyhow!(e))?
        .recv_json()
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(resp.token)
}

/// returns xsts_token and user_hash
async fn authenticate_with_xsts(xbl_token: &str) -> Result<(String, String)> {
    const AUTH_URL: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";

    #[derive(Deserialize)]
    struct Xui {
        uhs: String,
    }

    #[derive(Deserialize)]
    struct DisplayClaims {
        xui: Vec<Xui>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Response {
        token: String,
        display_claims: DisplayClaims,
    }

    let query = json!({
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [
                xbl_token
            ]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
    });

    let resp: Response = surf::post(AUTH_URL)
        .body_json(&query)
        .map_err(|e| anyhow!(e))?
        .recv_json()
        .await
        .map_err(|e| anyhow!(e))?;

    let user_hash = resp.display_claims.xui[0].uhs.clone();

    Ok((resp.token, user_hash))
}

/// returns mc_access_token
async fn authenticate_with_minecraft(xsts_token: &str, user_hash: &str) -> Result<String> {
    const AUTH_URL: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";

    #[derive(Deserialize)]
    struct Response {
        access_token: String,
    }

    let query = json!({ "identityToken": format!("XBL3.0 x={user_hash};{xsts_token}") });

    let resp: Response = surf::post(AUTH_URL)
        .body_json(&query)
        .map_err(|e| anyhow!(e))?
        .recv_json()
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(resp.access_token)
}

async fn get_minecraft_access_token(ms_access_token: &str) -> Result<String> {
    let xbl_token = authenticate_with_xbl(ms_access_token).await?;
    let (xsts_token, user_hash) = authenticate_with_xsts(&xbl_token).await?;
    let mc_access_token = authenticate_with_minecraft(&xsts_token, &user_hash).await?;

    Ok(mc_access_token)
}

#[derive(Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
}

/// returns user profile and access token
pub async fn get_user_profile(account: &Account) -> Result<UserProfile> {
    const PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

    let mc_access_token = get_minecraft_access_token(&account.access_token).await?;

    let resp: UserProfile = surf::get(PROFILE_URL)
        .header("Authorization", &format!("Bearer {mc_access_token}"))
        .recv_json()
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(resp)
}

pub async fn list() -> Result<Vec<(Account, UserProfile)>> {
    let accounts = read()?.accounts;
    let mut list = Vec::new();

    for account in accounts {
        let profile = get_user_profile(&account).await?;
        list.push((account, profile));
    }

    Ok(list)
}
