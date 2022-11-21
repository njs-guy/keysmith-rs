use crate::print_msg::print_msg;
use keysmith::timestamp;

pub struct TestTimestampOpts {
    pub string: bool,
    pub i64: bool,
    pub custom: bool,
    pub i64_custom: bool,
    pub utc: bool,
    pub utc_i64: bool,
}

pub fn test_timestamp(opts: TestTimestampOpts) {
    if opts.string {
        print_msg("Timestamp as String:");
        let stamp: String = timestamp::get_timestamp();
        println!("{}", stamp);
    }

    if opts.i64 {
        print_msg("Timestamp as i64:");
        let stamp: i64 = timestamp::get_timestamp_i64();
        println!("{}", stamp);
    }

    if opts.custom {
        print_msg("Timestamp with custom epoch as String:");
        let epoch: &str = "Mon, 1 Jan 2018 00:00:00 +0000";
        let stamp: String = timestamp::get_timestamp_custom(epoch);
        println!("{}", stamp);
    }

    if opts.i64_custom {
        print_msg("Timestamp with custom epoch as i64:");
        let epoch: &str = "Mon, 1 Jan 2018 00:00:00 +0000";
        let stamp: i64 = timestamp::get_timestamp_i64_custom(epoch);
        println!("{}", stamp);
    }

    if opts.utc {
        print_msg("Timestamp with no epoch as String:");
        let stamp: String = timestamp::get_timestamp_utc();
        println!("{}", stamp);
    }

    if opts.utc_i64 {
        print_msg("Timestamp with no epoch as i64:");
        let stamp: i64 = timestamp::get_timestamp_utc_i64();
        println!("{}", stamp);
    }
}
