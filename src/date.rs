use chrono::{Local, NaiveDate, ParseError, ParseResult};
use std::fmt::Formatter;

#[derive(Copy, Clone, Debug)]
pub(super) struct Date(NaiveDate);

impl Default for Date {
    fn default() -> Self {
        Date {
            0: Local::today().naive_local(),
        }
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Date {
    type Err = ParseError;

    fn from_str(s: &str) -> ParseResult<Date> {
        match NaiveDate::from_str(s) {
            Ok(naive) => Ok(Date { 0: naive }),
            Err(err) => Err(err),
        }
    }
}

impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        date.0
    }
}

#[cfg(test)]
mod tests {
    use crate::date::Date;
    use chrono::Local;

    #[test]
    fn default() {
        assert_eq!(Date::default().0, Local::today().naive_local())
    }
}
