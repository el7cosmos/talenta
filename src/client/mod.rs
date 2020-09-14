use reqwest::blocking;
use serde::Deserialize;
use std::collections::HashMap;
use url::{ParseError, Url};

#[derive(Deserialize, Debug)]
pub(super) struct ApiResponse<T> {
    message: String,
    status: u16,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub(super) fn message(&self) -> &str {
        &self.message
    }
    pub(super) fn status(&self) -> u16 {
        self.status
    }
    pub(super) fn data(&self) -> &Option<T> {
        &self.data
    }
}

#[derive(Deserialize, Debug)]
pub(super) struct LoginData {
    token: String,
}

impl LoginData {
    pub(super) fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Default, Debug)]
pub(super) struct Client {
    client: blocking::Client,
}

impl Client {
    pub(super) fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<ApiResponse<LoginData>, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        map.insert("email", email);
        map.insert("password", password);
        Ok(self
            .client
            .post(Client::build_url("login")?)
            .json(&map)
            .send()?
            .json()?)
    }

    fn build_url(path: &str) -> Result<Url, ParseError> {
        const BASE_URL: &str = "https://api-mobile.talenta.co/api/v1/";
        let base = Url::parse(BASE_URL).expect("hardcoded URL is known to be valid");
        let url = base.join(path)?;

        Ok(url)
    }
}
