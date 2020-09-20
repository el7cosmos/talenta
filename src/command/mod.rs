mod attendance;
mod login;

use crate::client::Client;
use crate::command::attendance::Attendance;
use crate::command::login::Login;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
pub(super) struct RootOpts {
    #[structopt(skip)]
    theme: ColorfulTheme,
}

pub(super) trait Command {
    fn run(self, client: Client) -> Result<String>;
}

#[derive(StructOpt)]
#[structopt(global_settings(& [AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder, AppSettings::VersionlessSubcommands]))]
pub(super) enum RootCommand {
    Login(Login),
    Attendance(Attendance),
}

impl Command for RootCommand {
    fn run(self, client: Client) -> Result<String> {
        match self {
            Self::Login(login) => login.run(client),
            Self::Attendance(attendance) => attendance.run(client),
        }
    }
}
