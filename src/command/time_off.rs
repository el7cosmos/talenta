use crate::client::Client;
use ansi_term::Style;
use chrono::NaiveDate;

pub(crate) fn check_time_off(date: NaiveDate, client: &Client) -> anyhow::Result<()> {
    if crate::client::is_time_off(date, client)? {
        Err(anyhow::anyhow!(
            "You have time off on {}",
            Style::new().bold().paint(date.to_string())
        ))
    } else {
        Ok(())
    }
}
