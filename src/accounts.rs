use crate::BASE_DIR;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    thread,
    time::Duration, collections::HashMap,
};
use url::Url;

#[derive(Serialize, Deserialize)]
struct Account {
    access_token: String,
    refresh_token: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    accounts: HashMap<String, Account>,
}

const CLIENT_ID: &str = "2000ea79-d993-4591-b9c4-e678f82ae1db";
const SCOPE: &str = "XboxLive.signin offline_access https://graph.microsoft.com/user.read";

lazy_static! {
    static ref ACCOUNTS_PATH: PathBuf = BASE_DIR.join("accounts").with_extension("toml");
}

fn get_new_config() -> Config {
    Config { accounts: HashMap::new() }
}

fn write(config: &Config) -> Result<(), Box<dyn Error>> {
    let config = toml::to_string(config)?;
    fs::write(ACCOUNTS_PATH.as_path(), config)?;

    Ok(())
}

fn new() -> Result<Config, Box<dyn Error>> {
    let config = get_new_config();
    write(&config)?;

    Ok(config)
}

fn read() -> Result<Config, Box<dyn Error>> {
    if !Path::is_file(ACCOUNTS_PATH.as_path()) {
        return new();
    }

    let data = fs::read_to_string(ACCOUNTS_PATH.as_path())?;
    let config = toml::from_str(&data)?;

    Ok(config)
}

fn add(account: Account) -> Result<(), Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Response {
        mail: String,
    }

    let resp: Response = ureq::get("https://graph.microsoft.com/v1.0/me")
        .set("Authorization", &format!("Bearer {}", account.access_token))
        .call()?
        .into_json()?;

    let mut config = read()?;
    config.accounts.insert(resp.mail, account);

    write(&config)?;

    Ok(())
}

// https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
pub fn authorize_device() -> Result<(String, String, Url), Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Response {
        device_code: String,
        user_code: String,
        verification_uri: Url,
    }

    let resp: Response =
        ureq::post("https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode")
            .send_form(&[("client_id", CLIENT_ID), ("scope", SCOPE)])?
            .into_json()?;

    Ok((resp.device_code, resp.user_code, resp.verification_uri))
}

fn refresh_token(account: &Account) -> Result<(), Box<dyn Error>> {
    ureq::post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token").send_form(&[
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("refresh_token", &account.refresh_token),
        ("grant_type", "refresh_token"),
    ])?;

    Ok(())
}

/// returns xbl_token
fn authenticate_with_xbl(ms_access_token: &str) -> Result<String, Box<dyn Error>> {
    #[derive(Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Response {
        token: String,
    }

    let resp: Response = ureq::post("https://user.auth.xboxlive.com/user/authenticate")
        .set("Accept", "application/json")
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

    Ok(resp.token)
}

/// returns xsts_token and user_hash
fn authenticate_with_xsts(xbl_token: &str) -> Result<(String, String), Box<dyn Error>> {
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

    let mut resp: Response = ureq::post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .set("Accept", "application/json")
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

    Ok((resp.token, resp.display_claims.xui.remove(0).uhs))
}

/// returns mc_access_token
fn authenticate_with_minecraft(
    xsts_token: &str,
    user_hash: &str,
) -> Result<String, Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Response {
        access_token: String,
    }

    let resp: Response =
        ureq::post("https://api.minecraftservices.com/authentication/login_with_xbox")
            .set("Accept", "application/json")
            .send_json(ureq::json!({
                "identityToken": format!("XBL3.0 x={};{}", user_hash, xsts_token)
            }))?
            .into_json()?;

    Ok(resp.access_token)
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
        error: String,
    }

    loop {
        let auth = ureq::post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
            .send_form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", CLIENT_ID),
                ("device_code", device_code),
            ]);

        match auth {
            Ok(response) => {
                let account: Account = response.into_json()?;
                add(account)?;

                break;
            }
            Err(ureq::Error::Status(code, response)) => {
                let resp: AuthenticationErrorResponse = response.into_json()?;
                match resp.error.as_str() {
                    "authorization_pending" => thread::sleep(Duration::from_secs(5)),
                    _ => {
                        println!("Authentication error");
                        println!("{} {}", code, resp.error);
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

#[derive(Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub access_token: String,
}

/// returns user profile and access token
fn get_user_profile(account: &Account) -> Result<UserProfile, Box<dyn Error>> {
    let mc_access_token = get_minecraft_access_token(&account.access_token)?;

    #[derive(Deserialize)]
    struct Response {
        pub id: String,
        pub name: String,
    }

    let resp: Response = ureq::get("https://api.minecraftservices.com/minecraft/profile")
        .set("Authorization", &format!("Bearer {}", mc_access_token))
        .call()?
        .into_json()?;

    let user_profile = UserProfile {
        id: resp.id,
        name: resp.name,
        access_token: mc_access_token,
    };

    Ok(user_profile)
}

pub fn list() -> Result<Vec<String>, Box<dyn Error>> {
    let accounts = read()?.accounts;
    let emails = accounts.keys().cloned().collect();

    Ok(emails)
}
