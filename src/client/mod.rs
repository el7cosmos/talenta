use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use url::{ParseError, Url};

#[derive(Deserialize, Debug)]
pub(crate) struct ApiResponse<T> {
    pub(crate) message: String,
    pub(crate) status: u16,
    pub(crate) data: Option<T>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct LoginData {
    pub(crate) token: String,
}

pub(crate) fn login(
    email: &str,
    password: &str,
) -> Result<ApiResponse<LoginData>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("email", email);
    map.insert("password", password);

    let client = Client::new();

    Ok(client.post(build_url("login")?).json(&map).send()?.json()?)
}

fn build_url(path: &str) -> Result<Url, ParseError> {
    const BASE_URL: &str = "https://api-mobile.talenta.co/api/v1/";
    let base = Url::parse(BASE_URL).expect("hardcoded URL is known to be valid");
    let url = base.join(path)?;

    Ok(url)
}
