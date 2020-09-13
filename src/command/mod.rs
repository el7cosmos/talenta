use crate::client::Client;
use crate::Config;
use ansi_term::Colour;
use clap::{ArgMatches, Error, ErrorKind};
use dialoguer::theme::Theme;
use dialoguer::{Input, Password};
use reqwest::StatusCode;
use std::io::{self, Write};
use std::process;

pub(crate) struct Command<'a> {
    client: Client,
    config: Config,
    theme: &'a dyn Theme,
}

impl<'a> Command<'a> {
    pub(crate) fn new(client: Client, config: Config, theme: &'a dyn Theme) -> Self {
        Command {
            client,
            config,
            theme,
        }
    }

    pub(crate) fn login(&self, matches: &ArgMatches) {
        let email: String = match matches.is_present("email") {
            true => matches.value_of("email").unwrap().into(),
            false => Input::with_theme(self.theme)
                .with_prompt("email")
                .interact()
                .unwrap(),
        };

        let password: String = match matches.is_present("password") {
            true => matches.value_of("password").unwrap().into(),
            false => Password::with_theme(self.theme)
                .with_prompt("password")
                .interact()
                .unwrap(),
        };

        match self.client.login(&email, &password) {
            Ok(login) => {
                let status = StatusCode::from_u16(login.status).unwrap();
                match status.is_success() {
                    true => {
                        Config::with_token(login.data.unwrap().token)
                            .store()
                            .unwrap();

                        Command::success(&login.message)
                    }
                    false => {
                        Error::with_description(&login.message, ErrorKind::ValueValidation).exit()
                    }
                }
            }
            Err(err) => {
                Error::with_description(&err.to_string(), ErrorKind::ValueValidation).exit()
            }
        }
    }

    fn success(message: &str) -> ! {
        let out = io::stdout();
        writeln!(&mut out.lock(), "{}", Colour::Green.bold().paint(message))
            .expect("Error writing Error to stdout");
        process::exit(0)
    }
}
