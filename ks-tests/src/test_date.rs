use crate::print_msg::print_msg;
use keysmith::date;

pub struct TestDateOpts {
	pub datetime: bool,
	pub datetime_no_seps: bool,
	pub datetime_abbr_month: bool,
	pub datetime_full_month: bool,
}

pub fn test_date(opts: TestDateOpts) {
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

	if opts.datetime_abbr_month {
		print_msg("Datetime with an abbreviated month:");
		let date: String = date::datetime_abbr_month();
		println!("{date}");
	}

	if opts.datetime_full_month {
		print_msg("Datetime with a full month name:");
		let date: String = date::datetime_full_month();
		println!("{date}");
	}
}
