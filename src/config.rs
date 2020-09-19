use confy::ConfyError;
use serde::{Deserialize, Serialize};

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Default, Serialize, Deserialize, Debug)]
pub(super) struct Config {
    token: Option<String>,
}

impl Config {
    pub(super) fn load() -> Result<Self, ConfyError> {
        confy::load(PKG_NAME)
    }

    pub(super) fn with_token(token: String) -> Config {
        Config { token: Some(token) }
    }

    pub(super) fn store(&self) -> Result<(), ConfyError> {
        confy::store(PKG_NAME, self)
    }

    pub(super) fn token(self) -> Option<String> {
        self.token
    }
}
