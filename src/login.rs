use crate::client::Client;
use crate::config::Config;
use crate::Command;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use reqwest::StatusCode;
use structopt::clap::{Error, ErrorKind};
use structopt::StructOpt;

#[derive(Default, StructOpt)]
pub(super) struct Login {
    #[structopt(skip)]
    client: Client,
    #[structopt(skip)]
    theme: ColorfulTheme,

    #[structopt(long)]
    email: Option<String>,
    #[structopt(long)]
    password: Option<String>,
}

impl Command for Login {
    fn run(&self) {
        let email = self.email.clone().unwrap_or_else(|| {
            Input::with_theme(&self.theme)
                .with_prompt("email")
                .interact()
                .unwrap()
        });
        let password = self.password.clone().unwrap_or_else(|| {
            Password::with_theme(&self.theme)
                .with_prompt("password")
                .interact()
                .unwrap()
        });

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
