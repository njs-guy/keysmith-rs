use crate::print_msg::print_msg;
use keysmith::char;
use keysmith::uuid::UUID;

pub struct TestGenCharOpts {
	pub char: bool,
	pub uuid_v4: bool,
	pub uuid_n: bool,
	pub custom: bool,
}

pub fn test_gen_char(num_of_chars: u32, opts: TestGenCharOpts) {
	if opts.char {
		print_msg("Char:");

		for _n in 1..=num_of_chars {
			// Generate a char from any character except unsafe_sp_chars.

			let opts = char::GenCharOpts {
				nums: true,
				letters: true,
				upper: true,
				safe_sp_chars: true,
				unsafe_sp_chars: false,
			};

			let c: char = char::char(opts);
			println!("{}", c);
		}
	}

	if opts.uuid_v4 {
		print_msg("UUID (v4) chars:");

		for _n in 1..=num_of_chars {
			// Generate a char for uuid v4.
			let c: char = char::char_uuid(UUID::V4);
			println!("{}", c);
		}
	}

	if opts.uuid_n {
		print_msg("UUID (nonstandard) chars:");

		for _n in 1..=num_of_chars {
			// Generate a char for nonstandard uuid.
			let c: char = char::char_uuid(UUID::Nonstandard);
			println!("{}", c);
		}
	}

	if opts.custom {
		print_msg("Char from custom character set:");

		for _n in 1..=num_of_chars {
			// Generate a char from a custom character set.
			let charset = String::from("among_us");
			let c: char = char::char_custom(&charset);
			println!("{}", c);
		}
	}
}
