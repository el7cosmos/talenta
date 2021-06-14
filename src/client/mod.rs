use std::collections::HashMap;

use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDate, NaiveTime};
use reqwest::blocking;
use reqwest::Url;

use request::{Attendance, LiveAttendance};
use response::{Calendar, CalendarEventType, Data, HistoryRequest, Login, Response};

mod request;
mod response;

#[derive(Debug)]
pub struct Client {
    client: blocking::Client,
    token: Option<String>,
}

impl Default for Client {
    fn default() -> Self {
        let user_agent = format!(
            "{}/{} ({}; +{})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_HOMEPAGE"),
        );

        Self {
            client: blocking::Client::builder()
                .user_agent(user_agent)
                .build()
                .expect("Client::default()"),
            token: None,
        }
    }
}

impl Client {
    pub fn login(&self, email: &str, password: &str) -> Result<Response<Login>> {
        let mut map = HashMap::new();
        map.insert("email", email);
        map.insert("password", password);
        let response: Response<Login> = self
            .client
            .post(Client::build_url("login")?)
            .json(&map)
            .send()?
            .json()?;
        response.result()
    }

    pub fn attendance_request(
        &self,
        reason: &str,
        date: NaiveDate,
        checkin: Option<NaiveTime>,
        checkout: Option<NaiveTime>,
    ) -> Result<Response<Data>> {
        let json = Attendance {
            datepicker_request_submit: date.to_string(),
            hour_checkin: checkin.map(|time| time.format("%H").to_string()),
            minute_checkin: checkin.map(|time| time.format("%M").to_string()),
            hour_checkout: checkout.map(|time| time.format("%H").to_string()),
            minute_checkout: checkout.map(|time| time.format("%M").to_string()),
            reason: reason.into(),
            use_check_in: checkin.is_some(),
            use_check_out: checkout.is_some(),
        };

        match &self.token {
            None => Err(anyhow::anyhow!("Not logged in yet")),
            Some(token) => Ok(self
                .client
                .post(Client::build_url("attendance-request")?)
                .bearer_auth(token)
                .json(&json)
                .send()?
                .json()?),
        }
    }

    pub fn live_attendance(
        &self,
        status: &str,
        latitude: f64,
        longitude: f64,
        description: Option<String>,
    ) -> Result<Response<Data>> {
        let form = LiveAttendance {
            status: status.to_string(),
            latitude,
            longitude,
            description,
        };

        match &self.token {
            None => Err(anyhow::anyhow!("Not logged in yet")),
            Some(token) => Ok(self
                .client
                .post(Client::build_url("live-attendance")?)
                .bearer_auth(token)
                .form(&form)
                .send()?
                .json()?),
        }
    }

    fn calendar(&self, year: i32, month: u32, date: Option<u32>) -> Result<Response<Calendar>> {
        match &self.token {
            None => Err(anyhow::anyhow!("Not logged in yet")),
            Some(token) => {
                let (start_date, end_date) = Client::calendar_date(year, month, date);

                let mut url = Client::build_url("calendar")?;
                url.query_pairs_mut().extend_pairs(&[
                    ("startDate", start_date.to_string()),
                    ("endDate", end_date.to_string()),
                    ("month", month.to_string()),
                ]);

                let response = self
                    .client
                    .get(url)
                    .bearer_auth(token)
                    .send()?
                    .json::<Response<Calendar>>()?;
                response.result()
            }
        }
    }

    fn history_request_time_off(&self, year: i32, month: u32) -> Result<Response<HistoryRequest>> {
        let mut url = Client::build_url("history-request/timeoff")?;
        url.query_pairs_mut()
            .extend_pairs(&[("month", month.to_string()), ("year", year.to_string())]);

        match &self.token {
            None => Err(anyhow::anyhow!("Not logged in yet")),
            Some(token) => Ok(self.client.get(url).bearer_auth(token).send()?.json()?),
        }
    }

    pub fn token(&self) -> &Option<String> {
        &self.token
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.into());
    }

    fn build_url(path: &str) -> anyhow::Result<Url> {
        const BASE_URL: &str = "https://api-mobile.talenta.co/api/v1/";
        let url = Url::parse(BASE_URL)
            .expect("hardcoded URL is known to be valid")
            .join(path)?;

        Ok(url)
    }

    fn calendar_date(year: i32, month: u32, date: Option<u32>) -> (NaiveDate, NaiveDate) {
        match date {
            None => {
                let next_month = match month {
                    12 => NaiveDate::from_ymd(year + 1, 1, 1),
                    _ => NaiveDate::from_ymd(year, month + 1, 1),
                };
                (
                    NaiveDate::from_ymd(year, month, 1),
                    next_month - Duration::days(1),
                )
            }
            Some(date) => {
                let naive_date = NaiveDate::from_ymd(year, month, date);
                (naive_date, naive_date)
            }
        }
    }
}

pub fn is_holiday(date: NaiveDate, client: &Client) -> Result<bool> {
    let calendar = client.calendar(date.year(), date.month(), Some(date.day()))?;
    match calendar.data {
        None => return Ok(false),
        Some(data) => {
            for event in data.events {
                if let CalendarEventType::N = event.event_type {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

pub fn is_time_off(date: NaiveDate, client: &Client) -> Result<bool> {
    let history = client.history_request_time_off(date.year(), date.month())?;
    match history.data {
        None => Ok(false),
        Some(data) => {
            for time_off in data.time_off_request.data {
                if time_off.start_date <= date && time_off.end_date >= date {
                    return Ok(true);
                }
            }

            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use reqwest::Url;

    use crate::client::Client;
    use chrono::NaiveDate;

    #[test]
    fn build_url() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            Url::parse("https://api-mobile.talenta.co/api/v1/path")?,
            Client::build_url("path")?
        );
        Ok(())
    }

    #[test]
    fn calendar_date() {
        assert_eq!(
            (
                NaiveDate::from_ymd(2021, 12, 1),
                NaiveDate::from_ymd(2021, 12, 31)
            ),
            Client::calendar_date(2021, 12, None)
        );
    }
}
