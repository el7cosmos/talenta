use std::collections::HashMap;

use anyhow::Result;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::NaiveTime;
use reqwest::blocking;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::client::response::{Calendar, CalendarEventType, HistoryRequest, Login, Response};

mod response;

#[derive(Deserialize, Debug)]
pub struct ResponseData {}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct AttendanceRequestBody {
    datepicker_request_submit: String,
    hour_checkin: Option<String>,
    minute_checkin: Option<String>,
    hour_checkout: Option<String>,
    minute_checkout: Option<String>,
    reason: String,
    useCheckIn: bool,
    useCheckOut: bool,
}

#[derive(Serialize, Debug)]
struct LiveAttendanceRequestBody {
    status: String,
    latitude: f64,
    longitude: f64,
    description: Option<String>,
}

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
    ) -> Result<Response<ResponseData>> {
        let json = AttendanceRequestBody {
            datepicker_request_submit: date.to_string(),
            hour_checkin: checkin.map(|time| time.format("%H").to_string()),
            minute_checkin: checkin.map(|time| time.format("%M").to_string()),
            hour_checkout: checkout.map(|time| time.format("%H").to_string()),
            minute_checkout: checkout.map(|time| time.format("%M").to_string()),
            reason: reason.into(),
            useCheckIn: checkin.is_some(),
            useCheckOut: checkout.is_some(),
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
    ) -> Result<Response<ResponseData>> {
        let form = LiveAttendanceRequestBody {
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

    fn calendar(
        &self,
        year: i32,
        month: u32,
        start_date: u32,
        end_date: u32,
    ) -> Result<Response<Calendar>> {
        let start_date = NaiveDate::from_ymd(year, month, start_date);
        let end_date = NaiveDate::from_ymd(year, month, end_date);

        let mut url = Client::build_url("calendar")?;
        url.query_pairs_mut().extend_pairs(&[
            ("startDate", start_date.to_string()),
            ("endDate", end_date.to_string()),
            ("month", month.to_string()),
        ]);

        match &self.token {
            None => Err(anyhow::anyhow!("Not logged in yet")),
            Some(token) => Ok(self.client.get(url).bearer_auth(token).send()?.json()?),
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
}

pub fn is_holiday(date: NaiveDate, client: &Client) -> Result<bool> {
    let calendar = client.calendar(date.year(), date.month(), date.day(), date.day())?;
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
