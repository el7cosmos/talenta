use crate::{client, Config};
use ansi_term::Colour;
use clap::ArgMatches;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input, Password};
use std::process;
use user_error::{UserFacingError, UFE};

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
            if login.status == 200 {
                Config::with_token(login.data.unwrap().token)
                    .store()
                    .unwrap();
                println!("{}", Colour::Green.bold().paint(login.message));
                process::exit(exitcode::OK)
            } else {
                let status = reqwest::StatusCode::from_u16(login.status).unwrap();
                UserFacingError::new(status.to_string())
                    .reason(login.message)
                    .print();
                process::exit(exitcode::NOUSER)
            }
        }
        Err(err) => {
            UserFacingError::from(err).print();
            process::exit(exitcode::NOUSER);
        }
    }
}
