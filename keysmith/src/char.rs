//! Generate a single character for use in a key.
//!
//! This is the same module used internally to generate keys.
//! The other modules are recommended, but this is
//! for when/if you need more options than what's
//! here by default.

use crate::uuid::UUID;
use rand::Rng;

pub const NUMBERS: &str = "0123456789";
pub const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
pub const HEX_LETTERS: &str = "abcdef";
pub const SAFE_SP_CHARS: &str = "-_.()~@";
pub const UNSAFE_SP_CHARS: &str = r#"#%&*+={}\/<>?!$:'"`|"#;

// See this https://stackoverflow.com/a/40415059 for special chars
// Might want to look more into this later

/// Options for char()
#[derive(Debug, Copy, Clone)]
pub struct GenCharOpts {
	/// Generate numbers?
	pub nums: bool,
	/// Generate letters?
	pub letters: bool,
	/// Generate uppercase letters?
	pub upper: bool,
	/// Generate safe special characters?
	pub safe_sp_chars: bool,
	/// Generate unsafe special characters? (false is recommended)
	pub unsafe_sp_chars: bool,
}

/// Generate numbers and letters (no uppercase)
fn gen_uuid_nonstandard_char() -> char {
	let opts = GenCharOpts {
		nums: true,
		letters: true,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	char(opts)
}

/// Generate numbers or letters a-f
fn gen_uuid_v4_char() -> char {
	let mut charset = String::from("");

	charset.push_str(NUMBERS);
	charset.push_str(HEX_LETTERS);

	get_char_from_set(&charset)
}

/// Returns a random character from the given character set
fn get_char_from_set(charset: &str) -> char {
	let mut rng = rand::thread_rng();

	// get a rand index from chars
	let idx = rng.gen_range(0..charset.len());

	// get the value of the index
	let c = charset
		.chars()
		.nth(idx)
		.expect("Could not get value of char.");

	c
}

// Public API

pub fn get_charset_from_opts(opts: GenCharOpts) -> String {
	let mut charset = String::from("");

	// Set allowed characters
	if opts.nums {
		charset.push_str(NUMBERS);
	}

	if opts.letters {
		charset.push_str(LETTERS);

		// This is nested because if letters aren't allowed,
		// uppercase letters obviously aren't allowed either.
		if opts.upper {
			charset.push_str(&LETTERS.to_uppercase());
		}
	}

	if opts.safe_sp_chars {
		charset.push_str(SAFE_SP_CHARS);
	}

	if opts.unsafe_sp_chars {
		charset.push_str(UNSAFE_SP_CHARS);
	}

	charset
}

/// Generates a char for a key. Use GenCharOpts for options.
pub fn char(opts: GenCharOpts) -> char {
	let charset = get_charset_from_opts(opts);

	get_char_from_set(&charset)
}

/// Generates a UUID char for the specified version.
///
/// Version input should be either '4' or 'n'.
///
/// Returns '0' if the input is invalid.
pub fn char_uuid(version: UUID) -> char {
	match version {
		UUID::V4 => gen_uuid_v4_char(),
		UUID::Nonstandard => gen_uuid_nonstandard_char(),
	}
	// Returns result of match version
}

/// Returns a random character from a custom character set
pub fn char_custom(charset: &str) -> char {
	get_char_from_set(charset)
}

// Tests
// TODO: Ensure that uuid_v4_char() generates the correct characters.
// TODO: Ensure that uuid_nonstandard_char() generates the correct characters.

#[cfg(test)]
mod tests {
	use super::*;

	// TODO: reuse this function

	// Returns false if the key contains a character that
	// is not in the charset.
	fn test_correct_chars(key: String, charset: &str) -> bool {
		let mut result = true;

		for key_c in key.chars() {
			if !&charset.contains(key_c) {
				result = false;
			}
		}

		result
	}

	#[test]
	fn test_get_char_from_set() {
		let charset = "abc123";

		let mut success = true;

		for _i in 1..=10 {
			let char = get_char_from_set(charset);

			// TODO: Only check this if success is true
			success = test_correct_chars(String::from(char), charset)
		}

		if !success {
			panic!("get_char_from_set() generated an invalid character.");
		}
	}

	#[test]
	fn test_char() {
		let opts = GenCharOpts {
			nums: true,
			letters: true,
			upper: true,
			safe_sp_chars: false,
			unsafe_sp_chars: false,
		};

		let charset = get_charset_from_opts(opts);

		let mut success = true;

		for _i in 1..=10 {
			let char = char(opts);
			success = test_correct_chars(String::from(char), &charset)
		}

		if !success {
			panic!("char() generated an invalid character.");
		}
	}
}
