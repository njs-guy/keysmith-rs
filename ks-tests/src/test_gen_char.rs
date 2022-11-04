use keysmith::gen_char;

pub struct TestGenCharOpts {
	pub char: bool,
	pub uuid_v4: bool,
	pub uuid_n: bool,
}

pub fn test_gen_char(num_of_chars: u32, opts: TestGenCharOpts) {
	if opts.char {
		println!("");
		println!("Chars:");
		println!("");

		for _n in 1..=num_of_chars {
			// Generate a char from any character except unsafe_sp_chars.

			let opts = gen_char::GenCharOpts {
				nums: true,
				letters: true,
				upper: true,
				safe_sp_chars: true,
				unsafe_sp_chars: false,
			};

			let c: char = gen_char::gen_char(opts);
			println!("{}", c);
		}
	}

	if opts.uuid_v4 {
		println!("");
		println!("UUID (v4) chars:");
		println!("");

		for _n in 1..=num_of_chars {
			// Generate a char for uuid v4.
			let c: char = gen_char::gen_uuid_char('4');
			println!("{}", c);
		}
	}

	if opts.uuid_n {
		println!("");
		println!("UUID (nonstandard) chars:");
		println!("");

		for _n in 1..=num_of_chars {
			// Generate a char for nonstandard uuid.
			let c: char = gen_char::gen_uuid_char('n');
			println!("{}", c);
		}
	}
}