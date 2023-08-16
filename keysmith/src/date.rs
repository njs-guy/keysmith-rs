//! Generates a written date or time.

/// Generate a date or time timestamp from the fmt provided.
fn gen_date(fmt: &str) -> String {
	let dt = chrono::Local::now();
	let timestamp = dt.format(fmt);

	format!("{}", timestamp)
}

/// Generates a date from the
/// current local time.
///
/// Format = YY-mm-dd
///
/// ex: 2023-07-17
pub fn date() -> String {
	gen_date("%Y-%m-%d")
}

/// Generates a date from the
/// current local time separated by slashes.
///
/// Format = YY/mm/dd
///
/// ex: 2023/07/17
///
/// ***WARNING:*** Don't use slashes when creating file names.
pub fn date_slashes() -> String {
	gen_date("%Y/%m/%d")
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

/// Generates a datetime from the
/// current local time.
///
/// Format = YY-mm-dd--HH-MM-SS
///
/// ex: 2023-07-17T19:34:07
pub fn datetime() -> String {
	gen_date("%Y-%m-%dT%H:%M:%S")
}

/// Generates a datetime from the
/// current local time with nicer formatting.
///
/// Format = YY/mm/dd HH:MM:SS
///
/// ex: 2023/07/17 19:34:07
///
/// ***WARNING:*** Don't use slashes when creating file names.
pub fn datetime_pretty() -> String {
	gen_date("%Y/%m/%d %H:%M:%S")
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

// TODO: change time_colon() to time()
// TODO: make time_hyphen()

/// Gets the current local time in 24 hour format.
///
/// ex: 19-34-07
pub fn time() -> String {
	gen_date("%H-%M-%S")
}

/// Gets the current local time in 24 hour format.
/// Separated by colons.
///
/// ex: 19:34:07
pub fn time_colon() -> String {
	gen_date("%H:%M:%S")
}

/// Gets the current local time in 24 hour format
/// without separators.
///
/// format = HHMMSS
///
/// ex: 193407
pub fn time_no_seps() -> String {
	gen_date("%H%M%S")
}

/// Gets the current local time in 24 hour format without seconds.
///
/// ex: 19-34
pub fn time_no_secs() -> String {
	gen_date("%H-%M")
}

/// Gets the current local time in 24 hour format without seconds
/// separated by a colon.
///
/// ex: 19:34
pub fn time_no_secs_colon() -> String {
	gen_date("%H:%M")
}

/// Gets the current local time in 24 hour format without seconds
/// or separators.
///
/// format = HHMM
///
/// ex: 1934
pub fn time_no_secs_no_seps() -> String {
	gen_date("%H%M")
}

/// Generate a datetime timestamp with custom formatting.
///
/// ex: "%Y-%m-%d--%H-%M-%S" -> 2023-07-17--19-34-07
///
/// For formatting reference, check out the [chrono docs](https://docs.rs/chrono/latest/chrono/format/strftime/index.html).
pub fn datetime_custom(fmt: &str) -> String {
	gen_date(fmt)
}

// TODO: Tests to make sure that the date stamps use the right format.
