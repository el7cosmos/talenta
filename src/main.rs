use crate::command::{Command, RootCommand};
use ansi_term::Colour;
use std::io::{self, Write};
use std::process;
use structopt::StructOpt;

mod client;
mod command;
mod config;
mod date;
mod time;

fn main() {
    let client = client::Client::default();

    match RootCommand::from_args().run(client) {
        Ok(message) => {
            let out = io::stdout();
            writeln!(
                &mut out.lock(),
                "{} {}",
                Colour::Green.bold().paint("ok:"),
                message
            )
            .expect("Error writing to stdout");
            process::exit(0);
        }
        Err(error) => {
            let err = io::stderr();
            writeln!(
                &mut err.lock(),
                "{} {}",
                Colour::Red.bold().paint("error:"),
                error.to_string()
            )
            .expect("Error writing to stderr");
            process::exit(1)
        }
    }
}
