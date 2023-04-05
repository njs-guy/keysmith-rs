//! Generate a single character for use in a key.
//!
//! This is the same module used internally to generate keys.
//! The other modules are recommended, but this is
//! for when/if you need more options than what's
//! here by default.

mod possible_chars;
use possible_chars::{get_poss_chars, get_uuid_chars};

use rand::Rng;

// Gets a character set from possible_chars
// and then returns that for use in String.push_str()
fn push_poss_chars(char_set_name: &str) -> &str {
	let possible_chars = get_poss_chars();
	// TODO: Change this error message once the hashmaps have changed to pub consts.
	let expect_msg = format!(
		"Could not convert {} in possible_chars Hashmap.",
		char_set_name
	);

	return possible_chars.get(char_set_name).expect(&expect_msg);
}

// TODO: 0.4 - change doc comment to char()
/// Options for gen_char()
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

// Generate numbers and letters (no uppercase)
fn gen_uuid_nonstandard_char() -> char {
	let opts = GenCharOpts {
		nums: true,
		letters: true,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char(opts)
}

// Generate numbers or letters a-f
fn gen_uuid_v4_char() -> char {
	let mut chars = String::from("");
	let uuid_chars = get_uuid_chars();
	let expect_msg = "Could not convert uuid chars in Hashmap.";

	chars.push_str(uuid_chars.get("numbers").expect(expect_msg));
	chars.push_str(uuid_chars.get("letters").expect(expect_msg));

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
// TODO: 0.4 - API changes: gen_char() -> char()

/// Generates a char for a key. Use GenCharOpts for options.
pub fn gen_char(opts: GenCharOpts) -> char {
	let mut chars = String::from("");

	// Set allowed characters
	if opts.nums {
		chars.push_str(push_poss_chars("numbers"));
	}

	if opts.letters {
		chars.push_str(push_poss_chars("en_alphabet"));

		if opts.upper {
			chars.push_str(&push_poss_chars("en_alphabet").to_uppercase());
		}
	}

	if opts.safe_sp_chars {
		chars.push_str(push_poss_chars("safe_sp_chars"));
	}

	if opts.unsafe_sp_chars {
		chars.push_str(push_poss_chars("unsafe_sp_chars"));
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

// TODO: possibly use an enum for this instead.

/// Generates a UUID char for the specified version.
///
/// Version input should be either '4' or 'n'.
///
/// Returns '0' if the input is invalid.
pub fn gen_uuid_char(version: char) -> char {
	match version {
		'4' => gen_uuid_v4_char(),
		'n' => gen_uuid_nonstandard_char(),
		_ => '0',
	}
	// Returns result of match version
}

// Tests
// TODO: Ensure that push_poss_chars() returns the expected set of characters.
// TODO: Ensure that char() generates valid characters based on the options provided.
// TODO: Ensure that uuid_v4_char() generates the correct characters.
// TODO: Ensure that uuid_nonstandard_char() generates the correct characters.
