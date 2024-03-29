mod print_msg;
mod test_date;
mod test_gen_char;
mod test_key;
mod test_timestamp;
mod test_uuid;

fn main() {
	// The length of each key
	let num_of_keys = 10;

	// turn modules on/off
	let gen_char = true;
	let date = true;
	let keys = true;
	let timestamp = true;
	let uuid = true;

	// Options

	let start = std::time::Instant::now(); // Start timer

	// which char functions to test
	let char_opts = test_gen_char::TestGenCharOpts {
		char: true,
		uuid_v4: true,
		uuid_n: true,
		custom: true,
	};

	// which date functions to test
	let date_opts = test_date::TestDateOpts {
		date: true,
		date_slashes: true,
		datetime: true,
		datetime_pretty: true,
		datetime_no_seps: true,
		time: true,
		time_hyphen: true,
		time_no_seps: true,
		time_no_secs: true,
		time_no_secs_hyphen: true,
		time_no_secs_no_seps: true,
		custom: true,
		date_no_seps: true,
	};

	// which gen_key functions to test
	let gen_key_opts = test_key::TestKeyOpts {
		keys: true,
		nums: true,
		letters: true,
		letters_lower: true,
		letters_upper: true,
		nums_and_letters: true,
		nums_and_letters_lower: true,
		nums_and_letters_upper: true,
		special_chars: true,
		special_chars_unsafe: true,
		custom: true,
	};

	// which timestamp functions to test
	let timestamp_opts = test_timestamp::TestTimestampOpts {
		string: true,
		i64: true,
		utc: true,
		utc_i64: true,
		ms: true,
		ms_i64: true,
		custom: true,
		custom_i64: true,
		ms_custom: true,
		ms_custom_i64: true,
	};

	// which uuid functions to test
	let uuid_opts = test_uuid::TestUuidOpts {
		uuid4: true,
		uuidn: true,
		uuid4_no_seps: true,
		uuidn_no_seps: true,
	};

	if gen_char {
		test_gen_char::test_gen_char(num_of_keys, char_opts);
	}

	if date {
		test_date::test_date(date_opts);
	}

	if keys {
		test_key::test_key_gen(num_of_keys, gen_key_opts);
	}

	if timestamp {
		test_timestamp::test_timestamp(timestamp_opts);
	}

	if uuid {
		test_uuid::test_uuid(num_of_keys, uuid_opts);
	}

	let end = std::time::Instant::now(); // End timer

	let time = (end - start).as_millis();

	let seconds: f64 = (time as f64) / 1000.00;

	print_msg::print_msg(&format!("Ran operations in {} seconds.", seconds));
}
