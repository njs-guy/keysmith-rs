use crate::print_msg::print_msg;
use keysmith::date;

pub struct TestDateOpts {
	pub date: bool,
	pub date_no_seps: bool,
	pub datetime: bool,
	pub datetime_no_seps: bool,
	pub custom: bool,
	pub time: bool,
	pub time_no_secs: bool,
}

pub fn test_date(opts: TestDateOpts) {
	if opts.date {
		print_msg("Date:");
		let date: String = date::date(false);
		println!("{date}");

		print_msg("Date (with slashes):");
		let date2: String = date::date(true);
		println!("{date2}");
	}

	if opts.date_no_seps {
		print_msg("Date without separators:");
		let date: String = date::date_no_seps();
		println!("{date}");
	}

	if opts.datetime {
		print_msg("Datetime:");
		let date: String = date::datetime();
		println!("{date}");
	}

	if opts.datetime_no_seps {
		print_msg("Datetime without separators:");
		let date: String = date::datetime_no_seps();
		println!("{date}");
	}

	if opts.custom {
		print_msg("Datetime with custom formatting:");
		let fmt = "%Y %Y %Y";
		let date: String = date::datetime_custom(fmt);
		print!("{date}");
	}

	if opts.time {
		print_msg("Local time:");
		let date: String = date::time();
		println!("{date}");
	}

	if opts.time_no_secs {
		print_msg("Local time without seconds:");
		let date: String = date::time_no_secs();
		println!("{date}");
	}
}
