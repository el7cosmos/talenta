use crate::login::Login;
use ansi_term::Colour;
use std::io::Write;
use std::{io, process};
use structopt::clap::AppSettings;
use structopt::StructOpt;

mod client;
mod config;
mod login;

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
}

fn main() {
    let app: App = App::from_args();

    match app.cmd {
        RootCommand::Login(login) => login.run(),
    }
}
