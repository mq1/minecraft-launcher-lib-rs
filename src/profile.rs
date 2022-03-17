use crate::msa::Account;
use anyhow::Result;
use serde::Deserialize;
use serde_json::json;

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

	let token_type = &account.token_type;
    let mc_access_token = get_minecraft_access_token(&account.access_token).await?;

    let resp: UserProfile = surf::get(PROFILE_URL)
        .header("Authorization", &format!("{token_type} {mc_access_token}"))
        .recv_json()
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(resp)
}
