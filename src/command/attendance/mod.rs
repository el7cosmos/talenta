mod checkin;
mod checkout;

use super::attendance::checkin::Checkin;
use super::attendance::checkout::Checkout;
use super::Command;
use super::RootOpts;
use crate::date::Date;
use crate::time::Time;
use ansi_term::Colour;
use anyhow::{anyhow, Result};
use dialoguer::Input;
use structopt::StructOpt;
use talenta::client::Client;

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

    #[structopt(name = "description", short, long, visible_aliases = & ["notes", "reason"])]
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
                .with_prompt("Description")
                .interact()
                .unwrap()
        });

        let response = client.attendance_request(
            &reason,
            self.date.into(),
            checkin_time.into(),
            checkout_time.into(),
        )?;
        response.result().map(|response| response.message)
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
            None => {
                super::holiday::check_holiday(self.date.into(), client)?;
                super::time_off::check_time_off(self.date.into(), client)?;
                self.execute(client)
            }
            Some(cmd) => match cmd {
                AttendanceCommand::Checkin(checkin) => checkin.run(client),
                AttendanceCommand::Checkout(checkout) => checkout.run(client),
            },
        }
    }
}
