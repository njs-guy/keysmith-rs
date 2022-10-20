use keysmith;

fn main() {
    let num_of_keys = 10;

    let keys = false;
    let nums = true;
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
