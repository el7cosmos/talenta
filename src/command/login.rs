use super::Command;
use crate::client::Client;
use crate::command::RootOpts;
use crate::config::Config;
use anyhow::Result;
use dialoguer::{Input, Password};
use reqwest::StatusCode;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
pub(crate) struct Login {
    #[structopt(flatten)]
    opts: RootOpts,

    #[structopt(long, env, hide_env_values = true)]
    email: Option<String>,
    #[structopt(long, env, hide_env_values = true)]
    password: Option<String>,
}

impl Command for Login {
    fn run(self, client: &Client) -> Result<String> {
        let theme = self.opts.theme;

        let email = self.email.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("email")
                .interact()
                .unwrap()
        });

        let password = self.password.unwrap_or_else(|| {
            Password::with_theme(&theme)
                .with_prompt("password")
                .interact()
                .unwrap()
        });

        let login = client.login(&email, &password)?;
        let status = StatusCode::from_u16(login.status())?;

        if status.is_success() {
            Config::with_token(login.data().as_ref().unwrap().token().into()).store()?;
            return Ok(login.message().to_string());
        }
        Err(anyhow::anyhow!("{}", login.message()))
    }
}
