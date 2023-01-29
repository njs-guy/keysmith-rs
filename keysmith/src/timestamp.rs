//! Gets a timestamp.
//! This is in seconds since the first second of 2022,
//! a custom epoch, or from plain UTC.

use chrono::{DateTime, Utc};

const DEF_EPOCH: &str = "Sat, 1 Jan 2022 00:00:00 +0000";

// Generates a timestamp from the input epoch
// Format = "Sat, 1 Jan 2022 00:00:00 +0000"
fn gen_timestamp(epoch_str: &str) -> i64 {
	let epoch = DateTime::parse_from_rfc2822(epoch_str)
		.expect("ERROR: Could not get timestamp epoch.");

	let utc = Utc::now();
	utc.timestamp() - epoch.timestamp()
}

// Public API
// TODO: 0.3.2 - millisecond and nanosecond timestamps
// TODO: 0.3.2 - Date time timestamps (YYYYMMDDHHMM or YYYYMMDDHHMMSS)
// TODO: 0.4 - API changes: gen_uuid('4') -> uuid4()

/// Gets a timestamp from the seconds since 00:00:00 Jan 1, 2022 as a String.
pub fn get_timestamp() -> String {
	gen_timestamp(DEF_EPOCH).to_string()
}

/// Gets a timestamp from the seconds since 00:00:00 Jan 1, 2022 as an i64.
pub fn get_timestamp_i64() -> i64 {
	gen_timestamp(DEF_EPOCH)
}

/// Gets a timestamp from a custom epoch as a String.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
pub fn get_timestamp_custom(epoch: &str) -> String {
	gen_timestamp(epoch).to_string()
}

/// Gets a timestamp from a custom epoch as an i64.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
pub fn get_timestamp_custom_i64(epoch: &str) -> i64 {
	gen_timestamp(epoch)
}

/// Gets a timestamp since the first second of 1970 as a String.
pub fn get_timestamp_utc() -> String {
	let epoch = "Thu, 1 Jan 1970 00:00:00 +0000";
	gen_timestamp(epoch).to_string()
}

/// Gets a timestamp since the first second of 1970 as an i64.
pub fn get_timestamp_utc_i64() -> i64 {
	let epoch = "Thu, 1 Jan 1970 00:00:00 +0000";
	gen_timestamp(epoch)
}

// TODO: More tests to make sure that the format is correct. Need to allow custom epochs first.
