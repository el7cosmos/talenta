mod checkin;
mod checkout;

use super::attendance::checkin::Checkin;
use super::attendance::checkout::Checkout;
use super::Command;
use crate::client::Client;
use crate::command::RootOpts;
use crate::date::Date;
use crate::time::Time;
use ansi_term::Colour;
use anyhow::{anyhow, Result};
use dialoguer::Input;
use reqwest::StatusCode;
use structopt::StructOpt;

#[derive(StructOpt)]
enum AttendanceCommand {
    Checkin(Checkin),
    Checkout(Checkout),
}

#[derive(Default, StructOpt)]
#[structopt(about = "Request attendance")]
pub(crate) struct Attendance {
    #[structopt(flatten)]
    opts: RootOpts,

    #[structopt(long)]
    reason: Option<String>,
    #[structopt(
        default_value,
        long,
        help = "Effective date (YYYY-mm-dd)",
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

impl Attendance {
    fn execute(self, client: &Client) -> Result<String> {
        let theme = self.opts.theme;

        let checkin_time = self.checkin_time.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("Checkin time (HH:mm)")
                .interact()
                .unwrap()
        });

        let checkout_time = self.checkout_time.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("Checkout time (HH:mm)")
                .interact()
                .unwrap()
        });

        let reason = self.reason.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("Reason")
                .interact()
                .unwrap()
        });

        let response = client.attendance_request(
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
}

impl Command for Attendance {
    fn run(self, client: &Client) -> Result<String> {
        if client.token().is_none() {
            return Err(anyhow!(
                "Not logged in yet. Try {}",
                Colour::Blue.bold().paint("talenta login")
            ));
        }

        match self.cmd {
            None => self.execute(client),
            Some(cmd) => match cmd {
                AttendanceCommand::Checkin(checkin) => checkin.run(client),
                AttendanceCommand::Checkout(checkout) => checkout.run(client),
            },
        }
    }
}
