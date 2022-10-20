use keysmith;

fn main() {
    // The length of each key
    let num_of_keys = 10;

    // Turn each individual function test on or off
    let keys = false;
    let nums = false;
    let letters = true;
    let letters_lower = true;
    let letters_upper = true;
    let special_chars = true;
    let special_chars_unsafe = true;

    let uuid4 = false;
    let uuidn = false;

    if keys {
        println!("Keys:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_key(32);
            println!("{}", key);
        }
    }

    if nums {
        println!("Numbers only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_nums(32);
            println!("{}", key);
        }
    }

    if letters {
        println!("Letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_letters(32);
            println!("{}", key);
        }
    }

    if letters_lower {
        println!("Lowercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_letters_lower(32);
            println!("{}", key);
        }
    }

    if letters_upper {
        println!("Uppercase letters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_letters_upper(32);
            println!("{}", key);
        }
    }

    if special_chars {
        println!("Special characters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_special_chars(32);
            println!("{}", key);
        }
    }

    if uuid4 {
        println!("");
        println!("UUIDs (v4):");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_uuid('4');
            println!("{}", key);
        }
    }

    if uuidn {
        println!("");
        println!("UUIDs (nonstandard):");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_uuid('n');
            println!("{}", key);
        }
    }
}
