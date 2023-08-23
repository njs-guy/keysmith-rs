//! Utilities for testing.

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
