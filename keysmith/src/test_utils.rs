//! Utilities for testing.

use crate::uuid::UUID;

/// Returns false if the key contains a character that
/// is not in the charset.
#[allow(dead_code)]
pub fn test_correct_chars(key: String, charset: &str) -> bool {
	// This function is only used in tests,
	// so the allow dead code warning can be ignored.

	let mut result = true;

	for key_c in key.chars() {
		if !&charset.contains(key_c) {
			result = false;
		}
	}

	result
}

/// Returns false if the key contains a character that
/// is not in the charset.
#[allow(dead_code)]
pub fn split_uuid(key: &str) -> Vec<String> {
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

/// Returns false if the chunk contains a character that
/// is not in the charset.
pub fn test_correct_uuid_chars(version: UUID, chunk: String) -> bool {
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

	// Return result of test_correct_chars
	test_correct_chars(chunk, &charset)
}

/// Returns false if the key contains a character that
/// is not in the charset.
#[allow(dead_code)]
pub fn test_valid_uuid(version: UUID) -> bool {
	use crate::uuid::{uuid4, uuidn};

	let mut success = true;

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

		success = test_correct_uuid_chars(version, String::from(chunk));

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
