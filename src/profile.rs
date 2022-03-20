use anyhow::Result;
use chrono::{DateTime, Duration, Local};
use isahc::{RequestExt, Request, ReadResponseExt};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// returns xbl_token
fn authenticate_with_xbl(ms_access_token: &str) -> Result<String> {
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

    // fallback for https://github.com/algesten/ureq/issues/470
    let mut resp = Request::post(AUTH_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&query)?)?
        .send()?;
    let text = resp.text()?;
    let token = serde_json::from_str::<Response>(&text)?.token;

    Ok(token)
}

/// returns xsts_token and user_hash
fn authenticate_with_xsts(xbl_token: &str) -> Result<(String, String)> {
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

    let resp: Response = ureq::post(AUTH_URL).send_json(query)?.into_json()?;

    let user_hash = resp.display_claims.xui[0].uhs.clone();

    Ok((resp.token, user_hash))
}

#[derive(Serialize, Deserialize)]
pub struct McAccount {
    access_token: String,
    token_type: String,
    expires: DateTime<Local>,
}

/// returns mc_access_token
fn authenticate_with_minecraft(xsts_token: &str, user_hash: &str) -> Result<McAccount> {
    const AUTH_URL: &str = "https://api.minecraftservices.com/authentication/login_with_xbox";

    #[derive(Deserialize)]
    struct Response {
        access_token: String,
        token_type: String,
        expires_in: i64,
    }

    let query = json!({ "identityToken": format!("XBL3.0 x={user_hash};{xsts_token}") });

    let resp: Response = ureq::post(AUTH_URL).send_json(query)?.into_json()?;

    let minecraft_account = McAccount {
        access_token: resp.access_token,
        token_type: resp.token_type,
        expires: Local::now() + Duration::seconds(resp.expires_in),
    };

    Ok(minecraft_account)
}

pub fn get_minecraft_account(ms_access_token: &str) -> Result<McAccount> {
    let xbl_token = authenticate_with_xbl(ms_access_token)?;
    let (xsts_token, user_hash) = authenticate_with_xsts(&xbl_token)?;
    let minecraft_account = authenticate_with_minecraft(&xsts_token, &user_hash)?;

    Ok(minecraft_account)
}

#[derive(Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
}

/// returns user profile and access token
pub fn get_user_profile(mca: &McAccount) -> Result<UserProfile> {
    const PROFILE_URL: &str = "https://api.minecraftservices.com/minecraft/profile";

    let tt = &mca.token_type;
    let at = &mca.access_token;

    let resp: UserProfile = ureq::get(PROFILE_URL)
        .set("Authorization", &format!("{tt} {at}"))
        .call()?
        .into_json()?;

    Ok(resp)
}
