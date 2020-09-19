mod checkin;
mod checkout;

use super::attendance::checkin::Checkin;
use super::attendance::checkout::Checkout;
use super::Command;
use crate::client::Client;
use crate::config::Config;
use crate::date::Date;
use crate::time::Time;
use ansi_term::Colour;
use anyhow::{anyhow, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use reqwest::StatusCode;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum AttendanceCommand {
    Checkin(Checkin),
    Checkout(Checkout),
}

#[derive(Default, StructOpt)]
#[structopt(about = "Request attendance")]
pub(crate) struct Attendance {
    #[structopt(skip)]
    client: Client,
    #[structopt(skip)]
    theme: ColorfulTheme,

    #[structopt(long)]
    reason: Option<String>,
    #[structopt(
        default_value,
        long,
        help = "Effective date (yyyy-mm-dd)",
        value_name = "DATE"
    )]
    date: Date,
    #[structopt(long, value_name = "TIME")]
    checkin_time: Option<Time>,
    #[structopt(long, value_name = "TIME")]
    checkout_time: Option<Time>,

    #[structopt(subcommand)]
    cmd: Option<AttendanceCommand>,
}

impl Command for Attendance {
    fn run(&mut self) -> Result<String> {
        let config = Config::load()?;
        let token = config.token().ok_or(anyhow!(
            "Not logged in yet. Try {}",
            Colour::Blue.bold().paint("talenta login")
        ))?;
        self.client.set_token(&token);

        match self.cmd.as_ref() {
            None => {
                let checkin_time = self.checkin_time.unwrap_or_else(|| {
                    Input::with_theme(&self.theme)
                        .with_prompt("Checkin time (HH:mm)")
                        .interact()
                        .unwrap()
                });

                let checkout_time = self.checkout_time.unwrap_or_else(|| {
                    Input::with_theme(&self.theme)
                        .with_prompt("Checkout time (HH:mm)")
                        .interact()
                        .unwrap()
                });

                let reason = match &self.reason {
                    Some(reason) => reason.to_string(),
                    None => Input::with_theme(&self.theme)
                        .with_prompt("Reason")
                        .interact()?,
                };

                let response = self.client.attendance_request(
                    &reason,
                    self.date.into(),
                    checkin_time.into(),
                    checkout_time.into(),
                )?;
                let status = StatusCode::from_u16(response.status())?;
                match status.is_success() {
                    true => Ok(response.message().to_string()),
                    false => Err(anyhow!("{}", response.message())),
                }
            }
            Some(cmd) => match cmd {
                AttendanceCommand::Checkin(_checkin) => unimplemented!(),
                AttendanceCommand::Checkout(_checkout) => unimplemented!(),
            },
        }
    }
}
