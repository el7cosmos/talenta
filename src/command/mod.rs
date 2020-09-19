mod attendance;
mod login;

use crate::command::attendance::Attendance;
use crate::command::login::Login;
use anyhow::Result;
use structopt::StructOpt;

pub(super) trait Command {
    fn run(&mut self) -> Result<String>;
}

#[derive(StructOpt)]
pub(super) enum RootCommand {
    Login(Login),
    Attendance(Attendance),
}

impl Command for RootCommand {
    fn run(&mut self) -> Result<String> {
        match self {
            Self::Login(login) => login.run(),
            Self::Attendance(attendance) => attendance.run(),
        }
    }
}
