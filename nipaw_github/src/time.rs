use jiff::{Timestamp, civil::DateTime, tz::TimeZone};

pub(crate) fn parse_datetime(s: &str) -> DateTime {
	s.parse::<Timestamp>().expect("invalid timestamp").to_zoned(TimeZone::UTC).datetime()
}

pub(crate) fn format_datetime(dt: DateTime) -> String {
	jiff::fmt::strtime::format("%Y-%m-%dT%H:%M:%SZ", dt).expect("invalid datetime format")
}
