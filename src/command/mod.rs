mod attendance;
mod login;

use crate::command::attendance::Attendance;
use crate::command::login::Login;
use ansi_term::Colour;
use anyhow::Result;
use std::io::Write;
use std::{io, process};
use structopt::StructOpt;

pub(super) trait Command {
    fn run(&self) -> Result<String>;

    fn success(message: &str) -> ! {
        let out = io::stdout();
        writeln!(&mut out.lock(), "{}", Colour::Green.bold().paint(message))
            .expect("Error writing Error to stdout");
        process::exit(0)
    }
}

#[derive(StructOpt)]
pub(super) enum RootCommand {
    Login(Login),
    Attendance(Attendance),
}

impl Command for RootCommand {
    fn run(&self) -> Result<String> {
        match self {
            Self::Login(login) => login.run(),
            Self::Attendance(attendance) => attendance.run(),
        }
    }
}
