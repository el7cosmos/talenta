use crate::command::{Command, RootCommand};
use structopt::clap::AppSettings;
use structopt::StructOpt;

mod client;
mod command;
mod config;
mod date;
mod time;

#[derive(StructOpt)]
#[structopt(global_settings(& [AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder, AppSettings::VersionlessSubcommands]))]
struct App {
    #[structopt(subcommand)]
    cmd: RootCommand,
}

fn main() {
    let app: App = App::from_args();
    app.cmd.run()
}
