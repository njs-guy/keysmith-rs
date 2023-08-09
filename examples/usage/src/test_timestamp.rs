use crate::print_msg::print_msg;
use keysmith::timestamp;

pub struct TestTimestampOpts {
	pub string: bool,
	pub i64: bool,
	pub utc: bool,
	pub utc_i64: bool,
	pub ms: bool,
	pub ms_i64: bool,
	pub custom: bool,
	pub custom_i64: bool,
	pub ms_custom: bool,
	pub ms_custom_i64: bool,
}

pub fn test_timestamp(opts: TestTimestampOpts) {
	let epoch: &str = "Mon, 1 Jan 2018 00:00:00 +0000";

	if opts.string {
		print_msg("Timestamp as String:");
		let stamp: String = timestamp::timestamp();
		println!("{}", stamp);
	}

	if opts.i64 {
		print_msg("Timestamp as i64:");
		let stamp: i64 = timestamp::timestamp_i64();
		println!("{}", stamp);
	}

	if opts.utc {
		print_msg("UTC timestamp as String:");
		let stamp: String = timestamp::timestamp_utc();
		println!("{}", stamp);
	}

	if opts.utc_i64 {
		print_msg("UTC timestamp as i64:");
		let stamp: i64 = timestamp::timestamp_utc_i64();
		println!("{}", stamp);
	}

	if opts.ms {
		print_msg("Millisecond timestamp as String:");
		let stamp: String = timestamp::timestamp_ms();
		println!("{}", stamp);
	}

	if opts.ms_i64 {
		print_msg("Millisecond timestamp as i64:");
		let stamp: i64 = timestamp::timestamp_ms_i64();
		println!("{}", stamp);
	}

	if opts.custom {
		print_msg("Timestamp with custom epoch as String:");
		let stamp: String = timestamp::timestamp_custom(epoch);
		println!("{}", stamp);
	}

	if opts.custom_i64 {
		print_msg("Timestamp with custom epoch as i64:");
		let stamp: i64 = timestamp::timestamp_custom_i64(epoch);
		println!("{}", stamp);
	}

	if opts.ms_custom {
		print_msg("Millisecond timestamp with custom epoch as String:");

		let stamp: String = timestamp::timestamp_ms_custom(epoch);
		println!("{}", stamp);
	}

	if opts.ms_custom_i64 {
		print_msg("Millisecond timestamp with custom epoch as i64:");
		let stamp: i64 = timestamp::timestamp_ms_custom_i64(epoch);
		println!("{}", stamp);
	}
}
