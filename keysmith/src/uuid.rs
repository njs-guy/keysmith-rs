//! Generates a UUID.
//!
//! Currently only version 4 or "nonstandard".
//!
//! Nonstandard is the same as v4 but uses a-z instead of just a-f.

use crate::char::char_uuid;

/// UUID version
#[derive(Debug, Clone, Copy)]
pub enum UUID {
	V4,
	Nonstandard,
}

/// Generate a UUID.
///
/// The version input should be a UUID enum.
///
/// v4 Ex: fc402d52-70be-7f09-caed-8da65db08985
///
/// nonstandard Ex: eko0c6ph-k2ok-60rr-pj78-mns182t9vurf
fn gen_uuid(version: UUID, separators: bool) -> String {
	let mut output = String::from("");
	let mut c: char;

	// first set of digits
	for _n in 1..=8 {
		c = char_uuid(version);

		output.push(c);
	}

	if separators {
		output.push('-'); // Current state: fc402d52-
	}

	// second through fourth sets of digits
	for _n in 1..=3 {
		for _x in 1..=4 {
			c = char_uuid(version);
			output.push(c);
		}

		if separators {
			output.push('-');
		}
		// After one loop: efc402d52-70be-
	}

	// Current state: fc402d52-70be-7f09-caed-

	// final set of digits
	for _n in 1..=12 {
		c = char_uuid(version);
		output.push(c);
	}

	output // fc402d52-70be-7f09-caed-8da65db08985
}

// Public API

/// Generate a UUID (version 4).
///
/// This is exactly the same as uuid4(),
/// but here for API simplicity.
///
/// ex: fc402d52-70be-7f09-caed-8da65db08985
pub fn uuid() -> String {
	uuid4()
}

/// Generate a UUID (version 4).
///
/// ex: fc402d52-70be-7f09-caed-8da65db08985
pub fn uuid4() -> String {
	gen_uuid(UUID::V4, true)
}

/// Generate a UUID (nonstandard).
/// This is the same as a v4, but letters can be
/// a-z instead of just a-f.
///
/// ex: l8fx3px5-9lyk-gzrb-iu75-d4gp63chor9z
pub fn uuidn() -> String {
	gen_uuid(UUID::Nonstandard, true)
}

/// Generate a UUID (version 4)
/// without separators.
///
/// This is exactly the same as uuid4_no_seps(),
/// but here for API simplicity.
///
/// ex: fc402d5270be7f09caed8da65db08985
pub fn uuid_no_seps() -> String {
	uuid4_no_seps()
}

/// Generate a UUID (version 4)
/// without separators.
///
/// ex: fc402d5270be7f09caed8da65db08985
pub fn uuid4_no_seps() -> String {
	gen_uuid(UUID::V4, false)
}

/// Generate a UUID (nonstandard)
/// without separators.
///
/// This is the same as a v4, but letters can be
/// a-z instead of just a-f.
///
/// ex: l8fx3px59lykgzrbiu75d4gp63chor9z
pub fn uuidn_no_seps() -> String {
	gen_uuid(UUID::Nonstandard, false)
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_utils::test_correct_chars;

	fn split_uuid(key: &str) -> Vec<String> {
		let mut result = Vec::new();
		let mut current_chunk = String::new();

		for c in key.chars() {
			if c == '-' {
				result.push(current_chunk.clone());
				current_chunk.clear();
			} else {
				current_chunk.push(c);
			}
		}

		if !current_chunk.is_empty() {
			result.push(current_chunk);
		}

		result
	}

	fn test_valid_uuid(version: UUID) -> bool {
		let mut success = true;

		let charset: String = match version {
			UUID::V4 => {
				let mut cs = String::from("");
				cs.push_str(crate::char::HEX_LETTERS);
				cs.push_str(crate::char::NUMBERS);

				cs
			}

			UUID::Nonstandard => {
				let mut cs = String::from("");
				cs.push_str(crate::char::LETTERS);
				cs.push_str(crate::char::NUMBERS);

				cs
			}
		};

		let key: String = match version {
			UUID::V4 => uuid4(),
			UUID::Nonstandard => uuidn(),
		};

		let chunks = &split_uuid(&key);

		let mut current_set = 1;

		for chunk in chunks {
			// Stop checking if the test has failed
			if !success {
				break;
			}

			success = test_correct_chars(String::from(chunk), &charset);

			// Number set 1 must be 8 characters.
			if current_set == 1 && chunk.len() != 8 {
				success = false;
			}

			// Number sets 2, 3, and 4 must be 4 characters.
			if (current_set == 2 || current_set == 3 || current_set == 4)
				&& chunk.len() != 4
			{
				success = false;
			}

			// Number set 5 must be 12 characters.
			if current_set == 5 && chunk.len() != 12 {
				success = false;
			}

			current_set += 1;
		}

		success
	}

	#[test]
	fn test_uuid4() {
		let success = test_valid_uuid(UUID::V4);

		if !success {
			panic!("uuid4() generated an invalid uuid.");
		}
	}

	#[test]
	fn test_uuidn() {
		let success = test_valid_uuid(UUID::Nonstandard);

		if !success {
			panic!("uuidn() generated an invalid uuid.");
		}
	}
}
