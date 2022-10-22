use keysmith;

fn main() {
    // The length of each key
    let num_of_keys = 10;

    // test readme code
    let readme_code = false;

    // which gen_key functions to test
    let gen_key_opts = TestKeyOpts {
        keys: true,
        nums: true,
        letters: true,
        letters_lower: true,
        letters_upper: true,
        nums_and_letters: true,
        nums_and_letters_lower: true,
        nums_and_letters_upper: true,
        special_chars: true,
        special_chars_unsafe: true
    };

    test_key_gen(num_of_keys, gen_key_opts);

    // which uuid functions to test
    let uuid_opts = TestUuidOpts {
        uuid4: true,
        uuidn: true
    };

    test_uuid(num_of_keys, uuid_opts);

    if readme_code {
        readme_code1();
        println!("");
        readme_code2();
        println!("");
        readme_code3();
        println!("");
        readme_code4();
    }
}

struct TestKeyOpts {
    keys: bool,
    nums: bool,
    letters: bool,
    letters_lower: bool,
    letters_upper: bool,
    nums_and_letters: bool,
    nums_and_letters_lower: bool,
    nums_and_letters_upper: bool,
    special_chars: bool,
    special_chars_unsafe: bool
}

fn test_key_gen(num_of_keys: u32, opts: TestKeyOpts) {

    if opts.keys {
        println!("Keys:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_key(32);
            println!("{}", key);
        }
    }

    if opts.nums {
        println!("Numbers only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_nums(32);
            println!("{}", key);
        }
    }

    if opts.letters {
        println!("Letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_letters(32);
            println!("{}", key);
        }
    }

    if opts.letters_lower {
        println!("Lowercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_letters_lower(32);
            println!("{}", key);
        }
    }

    if opts.letters_upper {
        println!("Uppercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_letters_upper(32);
            println!("{}", key);
        }
    }

    if opts.nums_and_letters {
        println!("Numbers and letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_nums_and_letters(32);
            println!("{}", key);
        }
    }

    if opts.nums_and_letters_lower {
        println!("Numbers and lowercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_nums_and_letters_lower(32);
            println!("{}", key);
        }
    }

    if opts.nums_and_letters_upper {
        println!("Numbers and lowercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_nums_and_letters_upper(32);
            println!("{}", key);
        }
    }

    if opts.special_chars {
        println!("Special characters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_special_chars(32);
            println!("{}", key);
        }
    }

    if opts.special_chars_unsafe {
        println!("Unsafe special characters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_special_chars_unsafe(32);
            println!("{}", key);
        }
    }
}

struct TestUuidOpts {
    uuid4: bool,
    uuidn: bool,
}

fn test_uuid(num_of_keys: u32, opts: TestUuidOpts) {
    if opts.uuid4 {
        println!("");
        println!("UUIDs (v4):");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_uuid('4');
            println!("{}", key);
        }
    }

    if opts.uuidn {
        println!("");
        println!("UUIDs (nonstandard):");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_uuid('n');
            println!("{}", key);
        }
    }
}

// runs readme code to make sure it works
fn readme_code1() {
    println!("First code block in readme");
    println!("");

    let key1: String = keysmith::gen_key(64);
    println!("{}", key1);
    // ex: aVCkmMFkv3UqLIU2tC52DQOWrIg4RsaH.xvTCTvA_PVHY@MOIWH~y1610KIZ@qM@

    let key2: String = keysmith::gen_uuid('4');
    println!("{}", key2);
    // ex: da748b8b-e915-661b-466a-0d6a0480112a

    let key3: String = keysmith::gen_uuid('n');
    println!("{}", key3);
    // "Nonstandard" UUID
    // ex: eko0c6ph-k2ok-60rr-pj78-mns182t9vurf
}

fn readme_code2() {
    println!("Second code block in readme");
    println!("");

    let key1: String = keysmith::gen_nums(32);
    println!("{}", key1);
    // ex: 04356417134317004828941212534445

    let key2: String = keysmith::gen_letters(32);
    println!("{}", key2);
    // ex: PBSZWwSTmRalGnzeSbQUKmXRikKUWXvj

    let key3: String = keysmith::gen_special_chars(32);
    println!("{}", key3);
    // Special characters generally considered "safe."
    // Possible characters: -_.()~@
    // ex: )@-_~@_@._))~)@))@.)(-)@(.@(~((@
}

fn readme_code3() {
    println!("Third code block in readme");
    println!("");

    let key1: String = keysmith::gen_letters_lower(32);
    println!("{}", key1);
    // ex: xoewhgvjsqzctfgpaqwnhanbgweflpqc

    let key2: String = keysmith::gen_letters_upper(32);
    println!("{}", key2);
    // ex: EVQMPIHKDBPLZJBPCHTXTIBLYRSFFFUY
}

fn readme_code4() {
    println!("Fourth code block in readme");
    println!("");

    let key: String = keysmith::gen_special_chars_unsafe(32);
    println!("{}", key);

    // ex: <#=`=*%{:`*%!<{"|*?'!#\#|?\+{=\}
}