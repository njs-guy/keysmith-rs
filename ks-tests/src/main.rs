mod print_msg;
mod test_gen_char;
mod test_key;
mod test_timestamp;
mod test_uuid;

fn main() {
	// The length of each key
	let num_of_keys = 10;

	// turn modules on/off
	let keys = true;
	let uuid = true;
	let timestamp = true;
	let gen_char = true;

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
	};

	// which uuid functions to test
	let uuid_opts = test_uuid::TestUuidOpts {
		uuid4: true,
		uuidn: true,
	};

	let timestamp_opts = test_timestamp::TestTimestampOpts {
		string: true,
		i64: true,
		custom: true,
		i64_custom: true,
		utc: true,
		utc_i64: true,
	};

	// which char functions to test
	let char_opts = test_gen_char::TestGenCharOpts {
		char: true,
		uuid_v4: true,
		uuid_n: true,
	};

	if keys {
		test_key::test_key_gen(num_of_keys, gen_key_opts);
	}

	if uuid {
		test_uuid::test_uuid(num_of_keys, uuid_opts);
	}

	if timestamp {
		test_timestamp::test_timestamp(timestamp_opts);
	}

	if gen_char {
		test_gen_char::test_gen_char(num_of_keys, char_opts)
	}
}
