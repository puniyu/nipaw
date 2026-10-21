use jiff::civil::DateTime;

pub(crate) fn format_datetime(dt: DateTime) -> String {
	jiff::fmt::strtime::format("%Y-%m-%dT%H:%M:%SZ", dt).expect("failed to format datetime")
}
