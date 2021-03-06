use std::convert::TryFrom;
use std::fmt::Formatter;

use chrono::NaiveDate;
use reqwest::StatusCode;
use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct Response<T> {
    pub message: String,
    #[serde(deserialize_with = "deserialize_status")]
    status: StatusCode,
    pub data: Option<T>,
}

impl<T> Response<T> {
    pub fn result(self) -> anyhow::Result<Self> {
        match self.status.is_success() {
            true => Ok(self),
            false => Err(anyhow::anyhow!(self.message)),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Data {}

#[derive(Deserialize, Debug)]
pub struct Login {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub enum CalendarEventType {
    B,
    N,
    T,
}

#[derive(Deserialize, Debug)]
pub struct CalendarEvent {
    pub title: String,
    start: NaiveDate,
    #[serde(rename = "type")]
    pub event_type: CalendarEventType,
    amount_days: String,
}

#[derive(Deserialize, Debug)]
pub struct Calendar {
    pub events: Vec<CalendarEvent>,
}

#[derive(Deserialize, Debug)]
pub struct TimeOffRequestData {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub struct TimeOffRequestList {
    pub data: Vec<TimeOffRequestData>,
}

#[derive(Deserialize, Debug)]
pub struct HistoryRequest {
    pub time_off_request: TimeOffRequestList,
}

fn deserialize_status<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    struct StatusCodeVisitor;

    impl<'de> Visitor<'de> for StatusCodeVisitor {
        type Value = StatusCode;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("number greater or equal to 100 and less than 1000")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            let unexpected = Unexpected::Unsigned(v);
            let code = u16::try_from(v).map_err(|_| Error::invalid_type(unexpected, &self))?;
            StatusCode::from_u16(code).map_err(|_| Error::invalid_type(unexpected, &self))
        }
    }

    deserializer.deserialize_u16(StatusCodeVisitor)
}
