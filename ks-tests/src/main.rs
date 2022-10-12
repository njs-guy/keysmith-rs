use keysmith;

fn main() {
    println!("Keys:");
    println!("");
    for _n in 1..=10 {
        let key: String = keysmith::gen_key(32);
        println!("{}", key);
    }

    println!("");
    println!("UUIDs (version 4):");
    println!("");
    for _n in 1..=10 {
        let key: String = keysmith::gen_uuid_v4();
        println!("{}", key);
    }
}
