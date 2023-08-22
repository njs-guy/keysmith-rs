//! Generates keys with specific configs.

use crate::char::{char_custom, get_charset_from_opts, GenCharOpts};

/// Generates a key from config opts
fn gen_char_from_opts(length: u32, opts: GenCharOpts) -> String {
	let charset = get_charset_from_opts(opts);
	gen_key(length, &charset) // Return generated key
}

fn gen_key(length: u32, charset: &str) -> String {
	let mut output = String::from("");
	for _n in 1..=length {
		let c = char_custom(charset);

		output.push(c);
	}

	output
}

// Public API

/// Generates a key string.
///
/// Ex: LlyqZk2W-Hm6Eoid~m(A8ymiM0q3ksyJ
pub fn key(length: u32) -> String {
	let opts = GenCharOpts {
		nums: true,
		letters: true,
		upper: true,
		safe_sp_chars: true,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string from a custom character set.
pub fn key_custom(length: u32, charset: String) -> String {
	gen_key(length, &charset)
}

/// Generates a key string using only numbers.
///
/// Ex: 04356417134317004828941212534445
pub fn nums(length: u32) -> String {
	let opts = GenCharOpts {
		nums: true,
		letters: false,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string using only letters
///
/// Ex: PBSZWwSTmRalGnzeSbQUKmXRikKUWXvj
pub fn letters(length: u32) -> String {
	let opts = GenCharOpts {
		nums: false,
		letters: true,
		upper: true,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string using only lowercase letters
///
/// Ex: xoewhgvjsqzctfgpaqwnhanbgweflpqc
pub fn letters_lower(length: u32) -> String {
	let opts = GenCharOpts {
		nums: false,
		letters: true,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string using only uppercase letters
///
/// Ex: EVQMPIHKDBPLZJBPCHTXTIBLYRSFFFUY
pub fn letters_upper(length: u32) -> String {
	let opts = GenCharOpts {
		nums: false,
		letters: true,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts).to_uppercase()
}

/// Generates a key string using only numbers and letters
///
/// ex: 2N1txo5sayvfaXIxreZMMpdKymewSHGL
pub fn nums_and_letters(length: u32) -> String {
	let opts = GenCharOpts {
		nums: true,
		letters: true,
		upper: true,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string using only numbers and lowercase letters
///
/// ex: ikmoc3lknebthl1xnb3crgu3qaav3f3f
pub fn nums_and_letters_lower(length: u32) -> String {
	let opts = GenCharOpts {
		nums: true,
		letters: true,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string using only numbers and uppercase letters
///
/// ex: NMJMTS1YOFQSL3CXHT23CVSIYM9FRLMN
pub fn nums_and_letters_upper(length: u32) -> String {
	let opts = GenCharOpts {
		nums: true,
		letters: true,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts).to_uppercase()
}

/// Generates a key string using characters that are generally considered safe.
///
/// Possible characters: -_.()~@
///
/// ex:
/// ```text
/// )@-_~@_@._))~)@))@.)(-)@(.@(~((@
/// ```
pub fn special_chars(length: u32) -> String {
	let opts = GenCharOpts {
		nums: false,
		letters: false,
		upper: false,
		safe_sp_chars: true,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string using only "unsafe" characters.
///
/// ***WARNING:*** these characters often break file structures
/// and URL's which is why they are considered unsafe.
///
/// This kind of key is not recommended but may be useful to *someone.*
///
/// Possible characters:
/// ```text
/// #%&*+={}\/<>?!$:'"`|
/// ```
///
/// ex:
/// ```text
/// <#=`=*%{:`*%!<{"|*?'!#\#|?\+{=\}
/// ```
pub fn special_chars_unsafe(length: u32) -> String {
	let opts = GenCharOpts {
		nums: false,
		letters: false,
		upper: false,
		safe_sp_chars: false,
		unsafe_sp_chars: true,
	};

	gen_char_from_opts(length, opts)
}

// TODO: Test that the generated output is the correct length and contains only the expected characters

#[cfg(test)]
mod tests {
	use super::*;

	// Tests that the generated key is the correct length.
	#[test]
	fn test_length() {
		let length = 32;
		let key = key(length);

		assert_eq!(key.len(), 32);
	}

	enum FunctionType {
		Key,
		Nums,
		Letters,
		LettersLower,
		NumsAndLetters,
		NumsAndLettersLower,
		SpecialChars,
		SpecialCharsUnsafe,
	}

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

	fn test_key_from_opts(opts: GenCharOpts, fn_type: FunctionType) -> bool {
		let charset = get_charset_from_opts(opts);
		let length = 32;

		let key: String = match fn_type {
			FunctionType::Key => key(length),
			FunctionType::Nums => nums(length),
			FunctionType::Letters => letters(length),
			FunctionType::LettersLower => letters_lower(length),
			FunctionType::NumsAndLetters => nums_and_letters(length),
			FunctionType::NumsAndLettersLower => nums_and_letters_lower(length),
			FunctionType::SpecialChars => special_chars(length),
			FunctionType::SpecialCharsUnsafe => special_chars_unsafe(length),
		};

		test_correct_chars(key, &charset)
	}

	// Tests that the generated key uses the correct characters.

	#[test]
	fn test_correct_characters_key() {
		let opts = GenCharOpts {
			nums: true,
			letters: true,
			upper: true,
			safe_sp_chars: true,
			unsafe_sp_chars: false,
		};

		let result = test_key_from_opts(opts, FunctionType::Key);

		if !result {
			panic!("Key contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_nums() {
		let opts = GenCharOpts {
			nums: true,
			letters: false,
			upper: false,
			safe_sp_chars: false,
			unsafe_sp_chars: false,
		};

		let result = test_key_from_opts(opts, FunctionType::Nums);

		if !result {
			panic!("Nums contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_letters() {
		let opts = GenCharOpts {
			nums: false,
			letters: true,
			upper: true,
			safe_sp_chars: false,
			unsafe_sp_chars: false,
		};

		let result = test_key_from_opts(opts, FunctionType::Letters);

		if !result {
			panic!("Letters contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_letters_lower() {
		let opts = GenCharOpts {
			nums: false,
			letters: true,
			upper: false,
			safe_sp_chars: false,
			unsafe_sp_chars: false,
		};

		let result = test_key_from_opts(opts, FunctionType::LettersLower);

		if !result {
			panic!("LettersLower contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_letters_upper() {
		// Using a custom charset is set to ensure that uppercase
		// letters are generated and not lowercase letters.

		let upper_case = crate::char::LETTERS.to_uppercase();

		let result = test_correct_chars(letters_upper(32), &upper_case);

		if !result {
			panic!("LettersUpper contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_nums_and_letters() {
		let opts = GenCharOpts {
			nums: true,
			letters: true,
			upper: true,
			safe_sp_chars: false,
			unsafe_sp_chars: false,
		};

		let result = test_key_from_opts(opts, FunctionType::NumsAndLetters);

		if !result {
			panic!("NumsAndLetters contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_nums_and_letters_lower() {
		let opts = GenCharOpts {
			nums: true,
			letters: true,
			upper: false,
			safe_sp_chars: false,
			unsafe_sp_chars: false,
		};

		let result =
			test_key_from_opts(opts, FunctionType::NumsAndLettersLower);

		if !result {
			panic!("NumsAndLettersLower contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_nums_and_letters_upper() {
		// Using a custom charset is set to ensure that uppercase
		// letters are generated and not lowercase letters.

		let upper_case = crate::char::LETTERS.to_uppercase();
		let nums = crate::char::NUMBERS;

		// let charset = upper_case + nums;
		let mut charset = String::from("");
		charset.push_str(&upper_case);
		charset.push_str(nums);

		let result = test_correct_chars(nums_and_letters_upper(32), &charset);

		if !result {
			panic!("LettersUpper contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_special_chars() {
		let opts = GenCharOpts {
			nums: false,
			letters: false,
			upper: false,
			safe_sp_chars: true,
			unsafe_sp_chars: false,
		};

		let result = test_key_from_opts(opts, FunctionType::SpecialChars);

		if !result {
			panic!("SpecialChars contained an invalid character.");
		}
	}

	#[test]
	fn test_correct_characters_special_chars_unsafe() {
		let opts = GenCharOpts {
			nums: false,
			letters: false,
			upper: false,
			safe_sp_chars: false,
			unsafe_sp_chars: true,
		};

		let result = test_key_from_opts(opts, FunctionType::SpecialCharsUnsafe);

		if !result {
			panic!("SpecialCharsUnsafe contained an invalid character.");
		}
	}
}
