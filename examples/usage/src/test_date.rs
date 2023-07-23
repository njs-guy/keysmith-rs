use crate::print_msg::print_msg;
use keysmith::date;

pub struct TestDateOpts {
	pub date: bool,
	pub date_no_seps: bool,
	pub date_abbr_month: bool,
	pub date_full_month: bool,
	pub datetime: bool,
	pub datetime_no_seps: bool,
	pub datetime_abbr_month: bool,
	pub datetime_full_month: bool,
	pub custom: bool,
	pub time: bool,
	pub time_no_secs: bool,
	pub secs: bool,
	pub min: bool,
	pub hour: bool,
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

	if opts.date_abbr_month {
		print_msg("Date with an abbreviated month:");
		let date: String = date::date_abbr_month(false);
		println!("{date}");

		print_msg("Date with an abbreviated month (with slashes):");
		let date2: String = date::date_abbr_month(true);
		println!("{date2}");
	}

	if opts.date_full_month {
		print_msg("Date with a full month name:");
		let date: String = date::date_full_month(false);
		println!("{date}");

		print_msg("Date with a full month name(with slashes):");
		let date2: String = date::date_full_month(true);
		println!("{date2}");
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

	if opts.secs {
		print_msg("Current seconds from local time:");
		let date: String = date::secs();
		println!("{date}");
	}

	if opts.min {
		print_msg("Current minutes from local time:");
		let date: String = date::min();
		println!("{date}");
	}

	if opts.hour {
		print_msg("Current hour from local time:");
		let date: String = date::hour();
		println!("{date}");
	}
}
