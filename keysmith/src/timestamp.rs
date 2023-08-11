//! Gets a timestamp.
//! This is in seconds since the first second of 2022,
//! a custom epoch, or from plain UTC.

use chrono::{DateTime, Utc};

const DEFAULT_EPOCH: &str = "Sat, 1 Jan 2022 00:00:00 +0000";
const UTC_EPOCH: &str = "Thu, 1 Jan 1970 00:00:00 +0000";

/// Generates a timestamp from the input epoch
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000"
fn gen_timestamp(epoch_str: &str) -> i64 {
	let epoch = DateTime::parse_from_rfc2822(epoch_str)
		.expect("ERROR: Could not get timestamp epoch.");

	let now = Utc::now();
	now.timestamp() - epoch.timestamp()
}

fn gen_timestamp_ms(epoch_str: &str) -> i64 {
	let epoch = DateTime::parse_from_rfc2822(epoch_str)
		.expect("ERROR: Could not get timestamp epoch.");
	let now = Utc::now();

	now.timestamp_millis() - epoch.timestamp_millis()
}

// Public API

/// Gets a timestamp from the first second of 2022 as a String.
///
/// ex: 50546851
pub fn timestamp() -> String {
	timestamp_i64().to_string()
}

/// Gets a timestamp from the the first second of 2022 as an i64.
///
/// ex: 50546851
pub fn timestamp_i64() -> i64 {
	gen_timestamp(DEFAULT_EPOCH)
}

/// Gets a timestamp since the first second of 1970 as a String.
///
/// ex: 1691542051
pub fn timestamp_utc() -> String {
	timestamp_utc_i64().to_string()
}

/// Gets a timestamp since the first second of 1970 as an i64.
///
/// ex: 1691542051
pub fn timestamp_utc_i64() -> i64 {
	gen_timestamp(UTC_EPOCH)
}

/// Gets a timestamp from the first second of 2022 as a String
/// using milliseconds.
///
/// ex: 50546851978
pub fn timestamp_ms() -> String {
	timestamp_ms_i64().to_string()
}

/// Gets a timestamp from the first second of 2022 as an i64
/// using milliseconds.
///
/// ex: 50546851978
pub fn timestamp_ms_i64() -> i64 {
	gen_timestamp_ms(DEFAULT_EPOCH)
}

/// Gets a timestamp from a custom epoch as a String.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
///
/// ex: 176777251
pub fn timestamp_custom(epoch: &str) -> String {
	timestamp_custom_i64(epoch).to_string()
}

/// Gets a timestamp from a custom epoch as an i64.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
///
/// ex: 176777251
pub fn timestamp_custom_i64(epoch: &str) -> i64 {
	gen_timestamp(epoch)
}

/// Gets a timestamp from a custom epoch as a String
/// using milliseconds.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
///
/// ex: 176777251981
pub fn timestamp_ms_custom(epoch: &str) -> String {
	timestamp_ms_custom_i64(epoch).to_string()
}

/// Gets a timestamp from a custom epoch as an i64
/// using milliseconds.
///
/// Format = "Sat, 1 Jan 2022 00:00:00 +0000".
///
/// ex: 176777251981
pub fn timestamp_ms_custom_i64(epoch: &str) -> i64 {
	gen_timestamp_ms(epoch)
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;

	/// Ensure gen_timestamp() returns the correct timestamp:
	#[test]
	fn test_gen_timestamp() {
		// Define an example epoch string

		// Parse the epoch string into a DateTime object
		let epoch = DateTime::parse_from_rfc2822(DEFAULT_EPOCH)
			.expect("Failed to parse epoch string");

		// Call the function with the example epoch string
		let result = gen_timestamp(DEFAULT_EPOCH);

		// Calculate the expected result
		let now = Utc::now();
		let expected = now.timestamp() - epoch.timestamp();

		// Compare the actual result with the expected result
		assert_eq!(result, expected);
	}
}
