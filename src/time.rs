use chrono::{Local, NaiveTime, ParseError, ParseResult, Timelike};

#[derive(Copy, Clone, Debug)]
pub(super) struct Time(NaiveTime);

impl Default for Time {
    fn default() -> Self {
        Time {
            0: Local::now().with_nanosecond(0).unwrap().time(),
        }
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Time {
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<Time> {
        NaiveTime::parse_from_str(s, "%R")
            .or_else(|_| NaiveTime::parse_from_str(s, "%T"))
            .or_else(|_| NaiveTime::from_str(s))
            .map(|f| Time { 0: f })
    }
}

impl From<Time> for NaiveTime {
    fn from(time: Time) -> Self {
        time.0
    }
}

impl From<Time> for Option<NaiveTime> {
    fn from(time: Time) -> Self {
        Some(time.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::time::Time;
    use chrono::{Local, Timelike};

    #[test]
    fn default() {
        assert_eq!(
            Time::default().0,
            Local::now().with_nanosecond(0).unwrap().time()
        )
    }
}
