use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf}, thread, time::Duration,
};
use crate::util::get_base_dir;

#[derive(Serialize, Deserialize)]
pub struct Account {
    access_token: String,
    ms_refresh_token: String
}

#[derive(Serialize, Deserialize)]
struct Config {
    accounts: Vec<Account>,
}

const CLIENT_ID: &str = "2000ea79-d993-4591-b9c4-e678f82ae1db";

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

fn add(account: Account) -> Result<(), Box<dyn Error>> {
    let mut config = read()?;
    config.accounts.push(account);

    write(&config)?;

    Ok(())
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
pub fn authorize_device() -> Result<(String, String, String), Box<dyn Error>> {
    let resp: serde_json::Value = ureq::post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
        .send_form(&[
            ("client_id", CLIENT_ID),
            ("scope", "XboxLive.signin%20offline_access")
        ])?
        .into_json()?;

    let device_code = resp["device_code"].to_string();
    let user_code = resp["user_code"].to_string();
    let verification_uri = resp["verification_uri"].to_string();

    Ok((device_code, user_code, verification_uri))
}

fn authenticate_with_xbl(ms_access_token: &str) -> Result<String, Box<dyn Error>> {
    let resp: serde_json::Value = ureq::post("https://user.auth.xboxlive.com/user/authenticate")
        .send_json(ureq::json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={}", ms_access_token)
            },
            "RelyingParty": "http://auth.xboxlive.com",
            "TokenType": "JWT"
        }))?
        .into_json()?;

    let xbl_token = resp["Token"].to_string();

    Ok(xbl_token)
}

fn authenticate_with_xsts(xbl_token: &str) -> Result<(String, String), Box<dyn Error>> {
    let resp: serde_json::Value = ureq::post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .send_json(ureq::json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [
                    xbl_token
                ]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        }))?
        .into_json()?;

    let xsts_token = resp["Token"].to_string();
    let user_hash = resp["DisplayClaims"]["xui"][0]["uhs"].to_string();

    Ok((xsts_token, user_hash))
}

fn authenticate_with_minecraft(xsts_token: &str, user_hash: &str) -> Result<String, Box<dyn Error>> {
    let resp: serde_json::Value = ureq::post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .send_json(ureq::json!({
            "identityToken": format!("XBL3.0 x={};{}", user_hash, xsts_token)
        }))?
        .into_json()?;
    
    let mc_access_token = resp["access_token"].to_string();

    Ok(mc_access_token)
}

fn get_minecraft_access_token(ms_access_token: &str) -> Result<String, Box<dyn Error>> {
    let xbl_token = authenticate_with_xbl(ms_access_token)?;
    let (xsts_token, user_hash) = authenticate_with_xsts(&xbl_token)?;
    let mc_access_token = authenticate_with_minecraft(&xsts_token, &user_hash)?;

    Ok(mc_access_token)
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
pub fn authenticate(device_code: &str) -> Result<(), Box<dyn Error>> {
    #[derive(Deserialize)]
    struct AuthenticationErrorResponse {
        error: String
    }

    #[derive(Deserialize)]
    struct AuthenticationResponse {
        access_token: String,
        refresh_token: String,
    }

    loop {
        let auth = ureq::post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .send_form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", CLIENT_ID),
                ("device_code", device_code)
            ]);

        match auth
        {
            Ok(response) => {
                let resp: AuthenticationResponse = response.into_json()?;
                let mc_access_token = get_minecraft_access_token(&resp.access_token)?;

                add(Account {
                    access_token: mc_access_token,
                    ms_refresh_token: resp.refresh_token
                })?;

                break;
            },
            Err(ureq::Error::Status(_code, response)) => {
                let resp: AuthenticationErrorResponse = response.into_json()?;
                match resp.error.as_str() {
                    "authorization_pending" => {
                        thread::sleep(Duration::from_secs(5))
                    },
                    _ => {
                        println!("Authentication error");
                        // TODO handle other errors
                        // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code#expected-errors

                        break;
                    }
                }
            }
            Err(_) => {
                println!("Network Error");
                return Err(Box::new(auth.err().unwrap()));
            }
        }
    }

    Ok(())
}
