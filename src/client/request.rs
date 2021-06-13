use serde::Serialize;

#[derive(Serialize, Debug)]
pub(crate) struct Attendance {
    pub(crate) datepicker_request_submit: String,
    pub(crate) hour_checkin: Option<String>,
    pub(crate) minute_checkin: Option<String>,
    pub(crate) hour_checkout: Option<String>,
    pub(crate) minute_checkout: Option<String>,
    pub(crate) reason: String,
    #[serde(rename = "useCheckIn")]
    pub(crate) use_check_in: bool,
    #[serde(rename = "useCheckOut")]
    pub(crate) use_check_out: bool,
}

#[derive(Serialize, Debug)]
pub(crate) struct LiveAttendance {
    pub(crate) status: String,
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) description: Option<String>,
}
