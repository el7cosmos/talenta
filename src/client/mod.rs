use anyhow::Result;
use chrono::{NaiveDate, NaiveTime};
use reqwest::blocking;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub(super) struct ApiResponse<T> {
    message: String,
    status: u16,
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub(super) fn message(&self) -> &str {
        &self.message
    }
    pub(super) fn status(&self) -> u16 {
        self.status
    }
    pub(super) fn data(&self) -> &Option<T> {
        &self.data
    }
}

#[derive(Deserialize, Debug)]
pub(super) struct ResponseData {}

#[derive(Deserialize, Debug)]
pub(super) struct LoginData {
    token: String,
}

impl LoginData {
    pub(super) fn token(&self) -> &str {
        &self.token
    }
}

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

#[derive(Default, Debug)]
pub(super) struct Client {
    client: blocking::Client,
    token: Option<String>,
}

impl Client {
    pub(super) fn login(&self, email: &str, password: &str) -> Result<ApiResponse<LoginData>> {
        let mut map = HashMap::new();
        map.insert("email", email);
        map.insert("password", password);
        Ok(self
            .client
            .post(Client::build_url("login")?)
            .json(&map)
            .send()?
            .json()?)
    }

    pub(super) fn attendance_request(
        &self,
        reason: &str,
        date: NaiveDate,
        checkin: Option<NaiveTime>,
        checkout: Option<NaiveTime>,
    ) -> Result<ApiResponse<ResponseData>> {
        let json = AttendanceRequestBody {
            datepicker_request_submit: date.to_string(),
            hour_checkin: match checkin {
                Some(time) => Some(time.format("%H").to_string()),
                None => None,
            },
            minute_checkin: match checkin {
                Some(time) => Some(time.format("%M").to_string()),
                None => None,
            },
            hour_checkout: match checkout {
                Some(time) => Some(time.format("%H").to_string()),
                None => None,
            },
            minute_checkout: match checkout {
                Some(time) => Some(time.format("%M").to_string()),
                None => None,
            },
            reason: reason.into(),
            useCheckIn: checkin.is_some(),
            useCheckOut: checkout.is_some(),
        };

        let response = self
            .client
            .post(Client::build_url("attendance-request")?)
            .bearer_auth(
                self.token
                    .as_deref()
                    .ok_or(anyhow::anyhow!("Not logged in yet"))?,
            )
            .json(&json)
            .send()?;

        Ok(response.json()?)
    }

    pub(super) fn token(&self) -> &Option<String> {
        &self.token
    }

    pub(super) fn set_token(&mut self, token: &str) {
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
