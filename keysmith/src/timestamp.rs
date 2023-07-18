//! Gets a timestamp.
//! This is in seconds since the first second of 2022,
//! a custom epoch, or from plain UTC.

use chrono::{DateTime, Utc};

const DEF_EPOCH: &str = "Sat, 1 Jan 2022 00:00:00 +0000";

/// Generates a timestamp from the input epoch
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000"
fn gen_timestamp(epoch_str: &str) -> i64 {
	let epoch = DateTime::parse_from_rfc2822(epoch_str)
		.expect("ERROR: Could not get timestamp epoch.");

	let utc = Utc::now();
	utc.timestamp() - epoch.timestamp()
}

// Public API
// TODO: 0.4 - millisecond timestamps

/// Gets a timestamp from the first second of 2022 as a String.
///
/// ex: 48627921
pub fn timestamp() -> String {
	gen_timestamp(DEF_EPOCH).to_string()
}

/// Gets a timestamp from the the first second of 2022 as an i64.
///
/// ex: 48627921
pub fn timestamp_i64() -> i64 {
	gen_timestamp(DEF_EPOCH)
}

/// Gets a timestamp from a custom epoch as a String.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
///
/// ex: 174858321
pub fn timestamp_custom(epoch: &str) -> String {
	gen_timestamp(epoch).to_string()
}

/// Gets a timestamp from a custom epoch as an i64.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
///
/// ex: 174858321
pub fn timestamp_custom_i64(epoch: &str) -> i64 {
	gen_timestamp(epoch)
}

/// Gets a timestamp since the first second of 1970 as a String.
///
/// ex: 1689623121
pub fn timestamp_utc() -> String {
	let epoch = "Thu, 1 Jan 1970 00:00:00 +0000";
	gen_timestamp(epoch).to_string()
}

/// Gets a timestamp since the first second of 1970 as an i64.
///
/// ex: 1689623121
pub fn timestamp_utc_i64() -> i64 {
	let epoch = "Thu, 1 Jan 1970 00:00:00 +0000";
	gen_timestamp(epoch)
}

// Tests

// TODO: Ensure that timestamp() returns the correct unix timestamp.
// TODO: Ensure that each function returns the correct type
