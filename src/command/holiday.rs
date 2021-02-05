use crate::client::Client;
use ansi_term::Style;
use anyhow::Result;
use chrono::NaiveDate;

pub(crate) fn check_holiday(date: NaiveDate, client: &Client) -> Result<()> {
    if crate::client::is_holiday(date, client)? {
        return Err(anyhow::anyhow!(
            "Selected date {} is holiday",
            Style::new().bold().paint(date.to_string())
        ));
    }

    Ok(())
}
