use super::Command;
use crate::client::Client;
use crate::config::Config;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use reqwest::StatusCode;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
pub(crate) struct Login {
    #[structopt(skip)]
    client: Client,
    #[structopt(skip)]
    theme: ColorfulTheme,

    #[structopt(long, value_name = "EMAIL")]
    email: Option<String>,
    #[structopt(long, value_name = "PASSWORD")]
    password: Option<String>,
}

impl Command for Login {
    fn run(&mut self) -> Result<String> {
        let email = match &self.email {
            Some(email) => email.to_string(),
            None => Input::with_theme(&self.theme)
                .with_prompt("email")
                .interact()?,
        };
        let password = match &self.password {
            Some(password) => password.to_string(),
            None => Password::with_theme(&self.theme)
                .with_prompt("password")
                .interact()?,
        };

        let login = self.client.login(&email, &password)?;
        let status = StatusCode::from_u16(login.status())?;

        if status.is_success() {
            Config::with_token(login.data().as_ref().unwrap().token().into()).store()?;
            return Ok(login.message().to_string());
        }
        Err(anyhow::anyhow!("{}", login.message()))
    }
}
