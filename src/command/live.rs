use crate::command::{Command, RootOpts};
use chrono::Local;
use dialoguer::{Confirm, Input};
use structopt::StructOpt;
use talenta::client::Client;

const DEFAULT_LATITUDE: f64 = -6.2093236;
const DEFAULT_LONGITUDE: f64 = 106.8186946;

#[derive(StructOpt)]
pub(crate) struct Live {
    #[structopt(flatten)]
    opts: RootOpts,

    #[structopt(name = "type", possible_values = & ["checkin", "checkout"])]
    attendance_type: String,
    #[structopt(long, value_name = "FLOAT")]
    latitude: Option<f64>,
    #[structopt(long, value_name = "FLOAT")]
    longitude: Option<f64>,
    #[structopt(short, long, visible_aliases = & ["notes", "reason"])]
    description: Option<String>,
}

impl Command for Live {
    fn run(self, client: &Client) -> anyhow::Result<String> {
        super::holiday::check_holiday(Local::today().naive_local(), client)?;
        super::time_off::check_time_off(Local::today().naive_local(), client)?;

        let theme = self.opts.theme;

        let (lat, lng) = match self.latitude.is_none() || self.longitude.is_none() {
            true => get_location()?,
            false => (DEFAULT_LATITUDE, DEFAULT_LONGITUDE),
        };

        let latitude = self.latitude.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("Latitude")
                .default(lat)
                .interact()
                .unwrap()
        });

        let longitude = self.longitude.unwrap_or_else(|| {
            Input::with_theme(&theme)
                .with_prompt("Longitude")
                .default(lng)
                .interact()
                .unwrap()
        });

        if !self.opts.interactive
            || Confirm::with_theme(&theme)
                .with_prompt(format!("Live checkin from: {},{}?", latitude, longitude))
                .interact()?
        {
            let response = client.live_attendance(
                &self.attendance_type,
                latitude,
                longitude,
                self.description,
            )?;
            return response.result().map(|response| response.message);
        }

        std::process::exit(1);
    }
}

fn get_location() -> anyhow::Result<(f64, f64)> {
    let client = reqwest::blocking::Client::new();

    let loc = match client.get("https://ipinfo.io/loc").send() {
        Ok(response) => response.text()?,
        Err(_) => format!("{},{}", DEFAULT_LATITUDE, DEFAULT_LONGITUDE),
    };

    let mut iter = loc.trim_end().split(',').map(str::parse::<f64>);

    Ok((iter.next().unwrap()?, iter.last().unwrap()?))
}
