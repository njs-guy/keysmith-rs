use keysmith::timestamp;

pub struct TestTimestampOpts {
	pub string: bool,
	pub i64: bool,
	pub custom: bool,
	pub i64_custom: bool,
}

pub fn test_timestamp(opts: TestTimestampOpts) {
	if opts.string {
		println!("Timestamp as String:");
		println!("");
		let stamp: String = timestamp::get_timestamp();
		println!("{}", stamp);
	}

	if opts.i64 {
		println!("Timestamp as i64:");
		println!("");
		let stamp: i64 = timestamp::get_timestamp_i64();
		println!("{}", stamp);
	}

	if opts.custom {
		println!("Timestamp with custom epoch as String:");
		println!("");
		let epoch: &str = "Mon, 1 Jan 2018 00:00:00 +0000";
		let stamp: String = timestamp::get_timestamp_custom(epoch);
		println!("{}", stamp);
	}

	if opts.i64_custom {
		println!("Timestamp with custom epoch as i64:");
		println!("");
		let epoch: &str = "Mon, 1 Jan 2018 00:00:00 +0000";
		let stamp: i64 = timestamp::get_timestamp_i64_custom(epoch);
		println!("{}", stamp);
	}
}