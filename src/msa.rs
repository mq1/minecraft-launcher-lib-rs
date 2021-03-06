use std::collections::HashMap;

use anyhow::Result;
use chrono::{prelude::*, Duration};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

const CLIENT_ID: &str = "2000ea79-d993-4591-b9c4-e678f82ae1db";
const SCOPE: &str = "XboxLive.signin offline_access";
const REDIRECT_URI: &str = "http://127.0.0.1:3003";

lazy_static! {
    static ref CODE_VERIFIER: String = {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect();

        rand_string
    };
    static ref CODE_CHALLENGE: String = {
        let hash = Sha256::digest(CODE_VERIFIER.as_str());
        let encoded = base64_url::encode(&hash);

        encoded
    };
    static ref STATE: String = {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        rand_string
    };
}

pub fn get_auth_url() -> Result<Url, url::ParseError> {
    Url::parse_with_params(
        "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize",
        &[
            ("client_id", CLIENT_ID),
            ("response_type", "code"),
            ("redirect_uri", REDIRECT_URI),
            ("response_mode", "query"),
            ("scope", SCOPE),
            ("state", STATE.as_ref()),
            ("code_challenge", CODE_CHALLENGE.as_ref()),
            ("code_challenge_method", "S256"),
        ],
    )
}

fn listen_login_callback() -> Result<String> {
    let server = tiny_http::Server::http("127.0.0.1:3003").unwrap();
    let request = server.recv()?;

    let url = Url::parse(&format!("{}{}", REDIRECT_URI, request.url()))?;
    let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();

    let state = hash_query
        .get("state")
        .ok_or(anyhow!("Auth2 state not found"))?;

    if state.ne(STATE.as_str()) {
        bail!("Invalid auth2 state");
    }

    let code = hash_query.get("code").ok_or(anyhow!("Code not found"))?;

    request.respond(tiny_http::Response::from_string("You can close this tab"))?;

    Ok(code.to_string())
}

#[derive(Serialize, Deserialize)]
pub struct MsAccount {
    pub access_token: String,
    pub token_type: String,
    expires: DateTime<Local>,
    refresh_token: String,
}

pub fn get_account() -> Result<MsAccount> {
    let code = listen_login_callback()?;

    #[derive(Deserialize)]
    struct Response {
        access_token: String,
        token_type: String,
        expires_in: i64,
        scope: String,
        refresh_token: String,
    }

    const URL: &str = "https://login.microsoftonline.com/consumers/oauth2/v2.0/token";

    let form = [
        ("client_id", CLIENT_ID),
        ("scope", SCOPE),
        ("code", &code),
        ("redirect_uri", REDIRECT_URI),
        ("grant_type", "authorization_code"),
        ("code_verifier", CODE_VERIFIER.as_ref()),
    ];

    let resp: Response = ureq::post(URL).send_form(&form)?.into_json()?;

    let token = MsAccount {
        access_token: resp.access_token,
        token_type: resp.token_type,
        expires: Local::now() + Duration::seconds(resp.expires_in),
        refresh_token: resp.refresh_token,
    };

    Ok(token)
}
