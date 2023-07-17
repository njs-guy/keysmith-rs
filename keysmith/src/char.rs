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
	let mut chars = String::from("");

	chars.push_str(NUMBERS);
	chars.push_str(HEX_LETTERS);

	let mut rng = rand::thread_rng();

	// Get a rand index from chars
	let idx = rng.gen_range(0..chars.len());

	// Get value of the index
	let c = chars
		.chars()
		.nth(idx)
		.expect("Could not get the value of the char.");

	c // return output as char
}

// Public API

/// Generates a char for a key. Use GenCharOpts for options.
pub fn char(opts: GenCharOpts) -> char {
	let mut chars = String::from("");

	// Set allowed characters
	if opts.nums {
		chars.push_str(NUMBERS);
	}

	if opts.letters {
		chars.push_str(LETTERS);

		if opts.upper {
			chars.push_str(&LETTERS.to_uppercase());
		}
	}

	if opts.safe_sp_chars {
		chars.push_str(SAFE_SP_CHARS);
	}

	if opts.unsafe_sp_chars {
		chars.push_str(UNSAFE_SP_CHARS);
	}

	let mut rng = rand::thread_rng();

	// get a rand index from chars
	let idx = rng.gen_range(0..chars.len());

	// get the value of the index
	let c = chars
		.chars()
		.nth(idx)
		.expect("Could not get value of char.");

	c // Return output as char
}

/// Generates a UUID char for the specified version.
///
/// Version input should be either '4' or 'n'.
///
/// Returns '0' if the input is invalid.
pub fn uuid_char(version: UUID) -> char {
	match version {
		UUID::V4 => gen_uuid_v4_char(),
		UUID::Nonstandard => gen_uuid_nonstandard_char(),
	}
	// Returns result of match version
}

// Tests
// TODO: Ensure that push_poss_chars() returns the expected set of characters.
// TODO: Ensure that char() generates valid characters based on the options provided.
// TODO: Ensure that uuid_v4_char() generates the correct characters.
// TODO: Ensure that uuid_nonstandard_char() generates the correct characters.
