use super::Command;
use crate::command::RootOpts;
use crate::config::Config;
use anyhow::Result;
use dialoguer::{Input, Password};
use structopt::StructOpt;
use talenta::client::Client;

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
        Config::with_token(login.data.unwrap().token).store()?;
        Ok(login.message)
    }
}
