use keysmith::timestamp;

pub struct TestTimestampOpts {
    pub string: bool,
    pub i64: bool,
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
}