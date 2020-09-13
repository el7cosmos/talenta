#[macro_use]
extern crate clap;

use crate::client::Client;
use crate::command::Command;
use confy::ConfyError;
use serde::{Deserialize, Serialize};

mod client;
mod command;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Default, Serialize, Deserialize, Debug)]
struct Config {
    token: String,
}

impl Config {
    fn load() -> Result<Self, ConfyError> {
        confy::load(PKG_NAME)
    }

    fn with_token(token: String) -> Config {
        Config { token }
    }

    fn store(&self) -> Result<(), ConfyError> {
        confy::store(PKG_NAME, self)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("../cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    let theme = dialoguer::theme::ColorfulTheme::default();
    let command = Command::new(
        Client::new(reqwest::blocking::Client::new()),
        Config::load()?,
        &theme,
    );

    match matches.subcommand_name() {
        Some("login") => command.login(matches.subcommand_matches("login").unwrap()),
        Some(&_) => {}
        None => {}
    }

    Ok(())
}
