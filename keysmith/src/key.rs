//! Generates keys with specific configs

use crate::gen_char::{gen_char, GenCharOpts};

fn gen_char_from_opts(length: u32, opts: GenCharOpts) -> String {
	let mut output = String::from("");
	for _n in 1..=length {
		let c = gen_char(opts);

		output.push(c);
	}

	output // Return output as String
}

// Public API
// TODO: 0.4 - API changes: gen_key() -> key()

/// Generates a key string.
///
/// Ex: LlyqZk2W-Hm6Eoid~m(A8ymiM0q3ksyJ
pub fn gen_key(length: u32) -> String {
	let opts = GenCharOpts {
		nums: true,
		letters: true,
		upper: true,
		safe_sp_chars: true,
		unsafe_sp_chars: false,
	};

	gen_char_from_opts(length, opts)
}

/// Generates a key string using only numbers.
///
/// Ex: 04356417134317004828941212534445
pub fn gen_nums(length: u32) -> String {
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
pub fn gen_letters(length: u32) -> String {
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
pub fn gen_letters_lower(length: u32) -> String {
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
pub fn gen_letters_upper(length: u32) -> String {
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
pub fn gen_nums_and_letters(length: u32) -> String {
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
pub fn gen_nums_and_letters_lower(length: u32) -> String {
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
pub fn gen_nums_and_letters_upper(length: u32) -> String {
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
/// ex: )@-_~@_@._))~)@))@.)(-)@(.@(~((@
pub fn gen_special_chars(length: u32) -> String {
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
pub fn gen_special_chars_unsafe(length: u32) -> String {
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

	// Tests that the generated key is the correct length
	#[test]
	fn test_length() {
		let length = 32;
		let key = gen_key(length);

		assert_eq!(key.len(), 32);
	}
}
