use crate::client::Client;
use crate::command::{Command, RootOpts};
use crate::date::Date;
use crate::time::Time;
use dialoguer::Input;
use reqwest::StatusCode;
use structopt::StructOpt;

#[derive(Default, StructOpt)]
#[structopt(about = "Request checkin only attendance")]
pub(super) struct Checkin {
    #[structopt(flatten)]
    opts: RootOpts,

    #[structopt(short, long)]
    reason: Option<String>,
    #[structopt(
        short,
        long,
        default_value,
        help = "Effective date (YYYY-mm-dd)",
        value_name = "DATE"
    )]
    date: Date,
    #[structopt(
        short,
        long,
        default_value,
        help = "Checkin time (HH:MM)",
        value_name = "TIME"
    )]
    time: Time,
}

impl Command for Checkin {
    fn run(self, client: Client) -> anyhow::Result<String> {
        let theme = self.opts.theme;

        let reason = self.reason.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("Reason")
                .interact()
                .unwrap()
        });

        let response =
            client.attendance_request(&reason, self.date.into(), self.time.into(), None)?;
        let status = StatusCode::from_u16(response.status())?;

        if status.is_success() {
            return Ok(response.message().to_string());
        }
        Err(anyhow::anyhow!("{}", response.message()))
    }
}
