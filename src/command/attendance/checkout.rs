use crate::command::{Command, RootOpts};
use crate::date::Date;
use crate::time::Time;
use dialoguer::Input;
use structopt::StructOpt;
use talenta::client::Client;

#[derive(Default, StructOpt)]
#[structopt(about = "Request checkin only attendance")]
pub(super) struct Checkout {
    #[structopt(flatten)]
    opts: RootOpts,

    #[structopt(name = "description", short, long, visible_aliases = & ["notes", "reason"])]
    reason: Option<String>,
    #[structopt(
        long,
        default_value,
        help = "Effective date (YYYY-mm-dd)",
        value_name = "DATE"
    )]
    date: Date,
    #[structopt(
        long,
        default_value,
        help = "Checkout time (HH:MM)",
        value_name = "TIME"
    )]
    time: Time,
}

impl Command for Checkout {
    fn run(self, client: &Client) -> anyhow::Result<String> {
        crate::command::holiday::check_holiday(self.date.into(), client)?;
        crate::command::time_off::check_time_off(self.date.into(), client)?;

        let theme = self.opts.theme;

        let reason = self.reason.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("Description")
                .interact()
                .unwrap()
        });

        let response =
            client.attendance_request(&reason, self.date.into(), None, self.time.into())?;
        response.result().map(|response| response.message)
    }
}
