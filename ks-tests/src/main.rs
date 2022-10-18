use keysmith;

fn main() {
    let num_of_keys = 10;

    println!("Keys:");
    println!("");
    for _n in 1..=num_of_keys {
        let key: String = keysmith::gen_key(32);
        println!("{}", key);
    }

    println!("");
    println!("UUIDs (v4):");
    println!("");
    for _n in 1..=num_of_keys {
        let key: String = keysmith::gen_uuid('4');
        println!("{}", key);
    }

    println!("");
    println!("UUIDs (nonstandard):");
    println!("");
    for _n in 1..=num_of_keys {
        let key: String = keysmith::gen_uuid('n');
        println!("{}", key);
    }
}
