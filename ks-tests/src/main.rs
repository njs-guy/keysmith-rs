use keysmith;

fn main() {
    // The length of each key
    let num_of_keys = 10;

    // Turn each individual function test on or off
    let keys = true;
    let nums = true;
    let letters = true;
    let letters_lower = true;
    let letters_upper = true;
    let special_chars = true;
    let special_chars_unsafe = true;

    let uuid4 = false;
    let uuidn = false;

    let readme_code = false;

    if readme_code {
        readme_code1();
        println!("");
        readme_code2();
        println!("");
        readme_code3();
        println!("");
        readme_code4();
    }

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

    if special_chars_unsafe {
        println!("Unsafe special characters only:");
        println!("");
        for _n in 1..=num_of_keys {
            let key: String = keysmith::gen_special_chars_unsafe(32);
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