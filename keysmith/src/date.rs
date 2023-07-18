//! Generates a date. Either written or as a number (returned as a String).

fn gen_date(fmt: &str) -> String {
	let dt = chrono::Local::now();
	let timestamp = dt.format(fmt);

	format!("{}", timestamp)
}

// TODO: Dates separated by forward slashes (date_cal)
// TODO: Times without a date

/// Generates a date from the
/// current local time.
///
/// Format = YY-mm-dd
///
/// ex: 2023-07-17
///
/// *WARNING:* Don't use slashes when creating file names.
pub fn date(slashes: bool) -> String {
	if slashes {
		gen_date("%Y/%m/%d")
	} else {
		gen_date("%Y-%m-%d")
	}
}

/// Generates a date from the
/// current local time without separators.
///
/// Format = YYmmdd
///
/// ex: 20230717
pub fn date_no_seps() -> String {
	gen_date("%Y%m%d")
}

/// Generates a date from the
/// current local time with an abbreviated month.
///
/// Format = YY-Mon-dd
///
/// ex: 2023-Jul-17
pub fn date_abbr_month(slashes: bool) -> String {
	if slashes {
		gen_date("%Y/%b/%d")
	} else {
		gen_date("%Y-%b-%d")
	}
}

/// Generates a date from the
/// current local time with a full month name.
///
/// Format = YY-Month-dd
///
/// ex: 2023-July-17
pub fn date_full_month(slashes: bool) -> String {
	if slashes {
		gen_date("%Y/%B/%d")
	} else {
		gen_date("%Y-%B-%d")
	}
}

// TODO: date_cal(slashes: true) => date_cal_slashes()

/// Generates a calendar date from the
/// current local time in dd/mm/YY format.
///
/// ex:
///
/// Use date::date_cal_mdy() if you're insane.
pub fn date_cal(slashes: bool) -> String {
	if slashes {
		gen_date("%d/%m/%Y")
	} else {
		gen_date("%d-%m-%Y")
	}
}

/// Generates a calendar date from the
/// current local time in year/day/month format.
///
/// ex:
pub fn date_cal_ydm(slashes: bool) -> String {
	if slashes {
		gen_date("%Y/%d/%m")
	} else {
		gen_date("%Y-%d-%m")
	}
}

/// Generates a calendar date from the
/// current local time in year/month/day format.
///
/// ex: 2023-07-17
pub fn date_cal_ymd(slashes: bool) -> String {
	if slashes {
		gen_date("%Y/%m/%d")
	} else {
		gen_date("%Y-%m-%d")
	}
}

/// Generates a calendar date from the
/// current local time in dd/mm/YY format.
///
/// ex:
///
/// Exactly the same as date::date_cal(), but here for code readability.
pub fn date_cal_dmy(slashes: bool) -> String {
	date_cal(slashes)
}

/// Generates a calendar date from the
/// current local time in mm/dd/YY format.
///
/// ex:
pub fn date_cal_mdy(slashes: bool) -> String {
	if slashes {
		gen_date("%m/%d/%Y")
	} else {
		gen_date("%m-%d-%Y")
	}
}

/// Generates a datetime from the
/// current local time.
///
/// Format = YY-mm-dd--HH-MM-SS
///
/// ex: 2023-07-17--19-34-07
pub fn datetime() -> String {
	gen_date("%Y-%m-%d--%H-%M-%S")
}

/// Generates a datetime from the
/// current local time without separators.
///
/// Format = YYmmddHHMMSS
///
/// ex: 20230717193407
pub fn datetime_no_seps() -> String {
	gen_date("%Y%m%d%H%M%S")
}

/// Generates a datetime from the
/// current local time with an abbreviated month.
///
/// Format = YY-Mon-dd--HH-MM-SS
///
/// ex: 2023-Jul-17--18-20-15
pub fn datetime_abbr_month() -> String {
	gen_date("%Y-%b-%d--%H-%M-%S")
}

/// Generates a datetime from the
/// current local time with a full month name.
///
/// Format = YY-Month-dd--HH-MM-SS
///
/// ex: 2023-July-17--18-20-15
pub fn datetime_full_month() -> String {
	gen_date("%Y-%B-%d--%H-%M-%S")
}
