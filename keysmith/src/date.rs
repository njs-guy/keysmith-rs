use chrono::Local;

fn gen_date(fmt: &str) -> String {
	let dt = Local::now();
	let timestamp = dt.format(fmt);

	format!("{}", timestamp)
}

// TODO: Dates without a time
// TODO: Times without a date

/// Generates a datetime timestamp from the
/// current local time.
///
/// Format = YY-mm-dd--HH-MM-SS
///
/// ex: 2023-07-17--19-34-07
pub fn datetime() -> String {
	gen_date("%Y-%m-%d--%H-%M-%S")
}

/// Generates a datetime timestamp from the
/// current local time without separators.
///
/// Format = YYmmddHHMMSS
///
/// ex: 20230717193407
pub fn datetime_no_seps() -> String {
	gen_date("%Y%m%d%H%M%S")
}

/// Generates a datetime timestamp from the
/// current local time with an abbreviated month.
///
/// Format = YY-Mon-dd--HH-MM-SS
///
/// ex: 2023-Jul-17--18-20-15
pub fn datetime_abbr_month() -> String {
	gen_date("%Y-%b-%d--%H-%M-%S")
}

/// Generates a datetime timestamp from the
/// current local time with a full month name.
///
/// Format = YY-Month-dd--HH-MM-SS
///
/// ex: 2023-July-17--18-20-15
pub fn datetime_full_month() -> String {
	gen_date("%Y-%B-%d--%H-%M-%S")
}
