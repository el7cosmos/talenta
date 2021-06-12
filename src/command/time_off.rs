use ansi_term::Style;
use chrono::NaiveDate;
use talenta::client::Client;

pub(crate) fn check_time_off(date: NaiveDate, client: &Client) -> anyhow::Result<()> {
    if talenta::client::is_time_off(date, client)? {
        Err(anyhow::anyhow!(
            "You have time off on {}",
            Style::new().bold().paint(date.to_string())
        ))
    } else {
        Ok(())
    }
}
