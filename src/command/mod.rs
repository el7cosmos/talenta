mod attendance;
mod holiday;
mod live;
mod login;

use crate::client::Client;
use crate::command::attendance::Attendance;
use crate::command::live::Live;
use crate::command::login::Login;
use anyhow::Result;
use dialoguer::theme::ColorfulTheme;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
pub(super) struct RootOpts {
    #[structopt(skip)]
    theme: ColorfulTheme,

    #[structopt(short = "n", long = "no-interaction", parse(from_flag = std::ops::Not::not))]
    interactive: bool,
}

pub(super) trait Command {
    fn run(self, client: &Client) -> Result<String>;
}

#[derive(StructOpt)]
#[structopt(global_settings(& [AppSettings::ColoredHelp, AppSettings::DeriveDisplayOrder, AppSettings::VersionlessSubcommands]))]
pub(super) enum RootCommand {
    Login(Login),
    Attendance(Box<Attendance>),
    Live(Live),
}

impl Command for RootCommand {
    fn run(self, client: &Client) -> Result<String> {
        match self {
            Self::Login(login) => login.run(client),
            Self::Attendance(attendance) => attendance.run(client),
            Self::Live(live) => live.run(client),
        }
    }
}
