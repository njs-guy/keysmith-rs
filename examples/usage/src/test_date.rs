use crate::print_msg::print_msg;
use keysmith::date;

pub struct TestDateOpts {
	pub date: bool,
	pub date_slashes: bool,
	pub date_no_seps: bool,
	pub datetime: bool,
	pub datetime_pretty: bool,
	pub datetime_no_seps: bool,
	pub time: bool,
	pub time_hyphen: bool,
	pub time_no_seps: bool,
	pub time_no_secs: bool,
	pub time_no_secs_hyphen: bool,
	pub time_no_secs_no_seps: bool,
	pub custom: bool,
}

pub fn test_date(opts: TestDateOpts) {
	if opts.date {
		print_msg("Date:");
		let date: String = date::date();
		println!("{date}");
	}

	if opts.date_slashes {
		print_msg("Date with slashes:");
		let date: String = date::date_slashes();
		println!("{date}");
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
	if opts.datetime_pretty {
		print_msg("Datetime (pretty):");
		let date: String = date::datetime_pretty();
		println!("{date}");
	}

	if opts.datetime_no_seps {
		print_msg("Datetime without separators:");
		let date: String = date::datetime_no_seps();
		println!("{date}");
	}

	if opts.time {
		print_msg("Local time:");
		let date: String = date::time();
		println!("{date}");
	}

	if opts.time_hyphen {
		print_msg("Local time separated by hyphens:");
		let date: String = date::time_hyphen();
		println!("{date}");
	}

	if opts.time_no_seps {
		print_msg("Local time without separators:");
		let date: String = date::time_no_seps();
		println!("{date}");
	}

	if opts.time_no_secs {
		print_msg("Local time without seconds:");
		let date: String = date::time_no_secs();
		println!("{date}");
	}

	if opts.time_no_secs_hyphen {
		print_msg("Local time without seconds, separated a hyphen:");
		let date: String = date::time_no_secs_hyphen();
		println!("{date}");
	}

	if opts.time_no_secs_no_seps {
		print_msg("Local time without seconds or separators:");
		let date: String = date::time_no_secs_no_seps();
		println!("{date}");
	}

	if opts.custom {
		print_msg("Datetime with custom formatting:");
		let fmt = "%Y %Y %Y";
		let date: String = date::datetime_custom(fmt);
		print!("{date}");
	}
}
