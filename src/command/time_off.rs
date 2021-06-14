use ansi_term::Style;
use chrono::{Datelike, NaiveDate};
use talenta::client::Client;

pub(crate) fn check_time_off(date: NaiveDate, client: &Client) -> anyhow::Result<()> {
    let history = client.history_request_time_off(date.year(), date.month())?;
    if let Some(data) = history.data {
        for time_off in data.time_off_request.data {
            if time_off.start_date <= date && time_off.end_date >= date {
                return Err(anyhow::anyhow!(
                    "You have time off on {}",
                    Style::new().bold().paint(date.to_string())
                ));
            }
        }
    }

    Ok(())
}
