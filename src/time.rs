use chrono::{Local, NaiveTime, ParseError, ParseResult};

#[derive(Copy, Clone, Debug)]
pub(super) struct Time(NaiveTime);

impl Default for Time {
    fn default() -> Self {
        Time {
            0: Local::now().time(),
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
        match NaiveTime::parse_from_str(s, "%R") {
            Ok(naive) => Ok(Time { 0: naive }),
            Err(_) => match NaiveTime::from_str(s) {
                Ok(naive) => Ok(Time { 0: naive }),
                Err(err) => Err(err),
            },
        }
    }
}

impl Into<NaiveTime> for Time {
    fn into(self) -> NaiveTime {
        self.0
    }
}

impl Into<Option<NaiveTime>> for Time {
    fn into(self) -> Option<NaiveTime> {
        Some(self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::time::Time;
    use chrono::{Local, Timelike};

    #[test]
    fn default() {
        assert_eq!(
            Time::default().0.with_nanosecond(0),
            Local::now().time().with_nanosecond(0)
        )
    }
}
