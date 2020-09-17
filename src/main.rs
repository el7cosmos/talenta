use crate::attendance::Attendance;
use crate::login::Login;
use ansi_term::Colour;
use std::io::Write;
use std::{io, process};
use structopt::clap::AppSettings;
use structopt::StructOpt;

mod attendance;
mod client;
mod config;
mod date;
mod login;
mod time;

trait Command {
    fn run(&self);

    fn success(message: &str) -> ! {
        let out = io::stdout();
        writeln!(&mut out.lock(), "{}", Colour::Green.bold().paint(message))
            .expect("Error writing Error to stdout");
        process::exit(0)
    }
}

#[derive(StructOpt)]
#[structopt(global_settings(& [AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder, AppSettings::VersionlessSubcommands]))]
struct App {
    #[structopt(subcommand)]
    cmd: RootCommand,
}

#[derive(StructOpt)]
enum RootCommand {
    Login(Login),
    Attendance(Attendance),
}

fn main() {
    let app: App = App::from_args();

    match app.cmd {
        RootCommand::Login(login) => login.run(),
        RootCommand::Attendance(attendance) => attendance.run(),
    }
}
