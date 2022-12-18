use crate::print_msg::print_msg;
use keysmith::uuid;

pub struct TestUuidOpts {
	pub uuid4: bool,
	pub uuidn: bool,
}

pub fn test_uuid(num_of_keys: u32, opts: TestUuidOpts) {
	if opts.uuid4 {
		print_msg("UUIDs (v4):");
		for _n in 1..=num_of_keys {
			let key: String = uuid::gen_uuid('4');
			println!("{}", key);
		}
	}

	if opts.uuidn {
		print_msg("UUIDs (nonstandard):");
		for _n in 1..=num_of_keys {
			let key: String = uuid::gen_uuid('n');
			println!("{}", key);
		}
	}
}
