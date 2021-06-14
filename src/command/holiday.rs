use ansi_term::Style;
use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use talenta::client::response::CalendarEventType;
use talenta::client::Client;

pub(crate) fn check_holiday(date: NaiveDate, client: &Client) -> Result<()> {
    let calendar = client.calendar(date.year(), date.month(), Some(date.day()))?;
    if let Some(data) = calendar.data {
        for event in data.events {
            if let CalendarEventType::N = event.event_type {
                return Err(anyhow::anyhow!(
                    "Selected date {} is a holiday: {}",
                    Style::new().bold().paint(date.to_string()),
                    Style::new().bold().paint(event.title)
                ));
            }
        }
    }

    Ok(())
}
