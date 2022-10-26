use keysmith::key;

pub struct TestKeyOpts {
    pub keys: bool,
    pub nums: bool,
    pub letters: bool,
    pub letters_lower: bool,
    pub letters_upper: bool,
    pub nums_and_letters: bool,
    pub nums_and_letters_lower: bool,
    pub nums_and_letters_upper: bool,
    pub special_chars: bool,
    pub special_chars_unsafe: bool
}

pub fn test_key_gen(num_of_keys: u32, opts: TestKeyOpts) {

    if opts.keys {
        println!("Keys:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_key(32);
            println!("{}", key);
        }
    }

    if opts.nums {
        println!("Numbers only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_nums(32);
            println!("{}", key);
        }
    }

    if opts.letters {
        println!("Letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_letters(32);
            println!("{}", key);
        }
    }

    if opts.letters_lower {
        println!("Lowercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_letters_lower(32);
            println!("{}", key);
        }
    }

    if opts.letters_upper {
        println!("Uppercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_letters_upper(32);
            println!("{}", key);
        }
    }

    if opts.nums_and_letters {
        println!("Numbers and letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_nums_and_letters(32);
            println!("{}", key);
        }
    }

    if opts.nums_and_letters_lower {
        println!("Numbers and lowercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_nums_and_letters_lower(32);
            println!("{}", key);
        }
    }

    if opts.nums_and_letters_upper {
        println!("Numbers and lowercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_nums_and_letters_upper(32);
            println!("{}", key);
        }
    }

    if opts.special_chars {
        println!("Special characters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_special_chars(32);
            println!("{}", key);
        }
    }

    if opts.special_chars_unsafe {
        println!("Unsafe special characters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = key::gen_special_chars_unsafe(32);
            println!("{}", key);
        }
    }
}