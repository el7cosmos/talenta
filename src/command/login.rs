use super::Command;
use crate::client::Client;
use crate::config::Config;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use reqwest::StatusCode;
use structopt::clap::{Error, ErrorKind};
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
    fn run(&self) {
        let email = match &self.email {
            Some(email) => email.to_string(),
            None => Input::with_theme(&self.theme)
                .with_prompt("email")
                .interact()
                .unwrap(),
        };
        let password = match &self.password {
            Some(password) => password.to_string(),
            None => Password::with_theme(&self.theme)
                .with_prompt("password")
                .interact()
                .unwrap(),
        };

        match self.client.login(&email, &password) {
            Ok(login) => {
                let status = StatusCode::from_u16(login.status()).unwrap();
                match status.is_success() {
                    true => {
                        Config::with_token(login.data().as_ref().unwrap().token().into())
                            .store()
                            .unwrap();

                        Login::success(&login.message())
                    }
                    false => {
                        Error::with_description(&login.message(), ErrorKind::ValueValidation).exit()
                    }
                }
            }
            Err(err) => {
                Error::with_description(&err.to_string(), ErrorKind::ValueValidation).exit()
            }
        }
    }
}
