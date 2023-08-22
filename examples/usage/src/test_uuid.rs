use crate::print_msg::print_msg;
use keysmith::uuid;

pub struct TestUuidOpts {
	pub uuid4: bool,
	pub uuidn: bool,
	pub uuid4_no_seps: bool,
	pub uuidn_no_seps: bool,
}

pub fn test_uuid(num_of_keys: u32, opts: TestUuidOpts) {
	if opts.uuid4 {
		print_msg("UUIDs (v4):");
		for _n in 1..=num_of_keys {
			let key: String = uuid::uuid4();
			println!("{}", key);
		}
	}

	if opts.uuidn {
		print_msg("UUIDs (nonstandard):");
		for _n in 1..=num_of_keys {
			let key: String = uuid::uuidn();
			println!("{}", key);
		}
	}

	if opts.uuid4_no_seps {
		print_msg("UUIDs (v4) without separators:");
		for _n in 1..=num_of_keys {
			let key: String = uuid::uuid4_no_seps();
			println!("{}", key);
		}
	}

	if opts.uuidn_no_seps {
		print_msg("UUIDs (nonstandard) without separators:");
		for _n in 1..=num_of_keys {
			let key: String = uuid::uuidn_no_seps();
			println!("{}", key);
		}
	}
}
