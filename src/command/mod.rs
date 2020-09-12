use crate::{client, Config};
use ansi_term::Colour;
use clap::{ArgMatches, Error, ErrorKind};
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use reqwest::StatusCode;
use std::io::{self, Write};
use std::process;

pub(crate) fn login(matches: &ArgMatches) {
    let theme = ColorfulTheme::default();

    let email: String = match matches.is_present("email") {
        true => matches.value_of("email").unwrap().into(),
        false => Input::with_theme(&theme)
            .with_prompt("email")
            .interact()
            .unwrap(),
    };

    let password: String = match matches.is_present("password") {
        true => matches.value_of("password").unwrap().into(),
        false => Password::with_theme(&theme)
            .with_prompt("password")
            .interact()
            .unwrap(),
    };

    match client::login(&email, &password) {
        Ok(login) => {
            let status = StatusCode::from_u16(login.status).unwrap();
            match status.is_success() {
                true => {
                    Config::with_token(login.data.unwrap().token)
                        .store()
                        .unwrap();

                    success(&login.message)
                }
                false => Error::with_description(&login.message, ErrorKind::ValueValidation).exit(),
            }
        }
        Err(err) => Error::with_description(&err.to_string(), ErrorKind::ValueValidation).exit(),
    }
}

fn success(message: &str) -> ! {
    let out = io::stdout();
    writeln!(&mut out.lock(), "{}", Colour::Green.bold().paint(message))
        .expect("Error writing Error to stdout");
    process::exit(0)
}
