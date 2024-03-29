use crate::print_msg::print_msg;
use keysmith::key;

pub struct TestKeyOpts {
	pub keys: bool,
	pub nums: bool,
	pub letters: bool,
	pub letters_lower: bool,
	pub letters_upper: bool,
	pub nums_and_letters: bool,
	pub nums_and_letters_lower: bool,
	pub nums_and_letters_upper: bool,
	pub special_chars: bool,
	pub special_chars_unsafe: bool,
	pub custom: bool,
}

pub fn test_key_gen(num_of_keys: u32, opts: TestKeyOpts) {
	if opts.keys {
		print_msg("Keys:");
		for _n in 1..=num_of_keys {
			let key: String = key::key(32);
			println!("{}", key);
		}
	}

	if opts.nums {
		print_msg("Numbers only:");
		for _n in 1..=num_of_keys {
			let key: String = key::nums(32);
			println!("{}", key);
		}
	}

	if opts.letters {
		print_msg("Letters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::letters(32);
			println!("{}", key);
		}
	}

	if opts.letters_lower {
		print_msg("Lowercase letters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::letters_lower(32);
			println!("{}", key);
		}
	}

	if opts.letters_upper {
		print_msg("Uppercase letters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::letters_upper(32);
			println!("{}", key);
		}
	}

	if opts.nums_and_letters {
		print_msg("Numbers and letters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::nums_and_letters(32);
			println!("{}", key);
		}
	}

	if opts.nums_and_letters_lower {
		print_msg("Numbers and lowercase letters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::nums_and_letters_lower(32);
			println!("{}", key);
		}
	}

	if opts.nums_and_letters_upper {
		print_msg("Numbers and uppercase letters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::nums_and_letters_upper(32);
			println!("{}", key);
		}
	}

	if opts.special_chars {
		print_msg("Special characters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::special_chars(32);
			println!("{}", key);
		}
	}

	if opts.special_chars_unsafe {
		print_msg("Unsafe special characters only:");
		for _n in 1..=num_of_keys {
			let key: String = key::special_chars_unsafe(32);
			println!("{}", key);
		}
	}

	if opts.custom {
		print_msg("Key from custom character set:");
		for _n in 1..=num_of_keys {
			let charset = String::from("among_us");
			let key: String = key::key_custom(32, charset);
			println!("{}", key);
		}
	}
}
