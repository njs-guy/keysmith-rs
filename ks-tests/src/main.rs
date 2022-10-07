use keysmith;

fn main() {
    for _n in 1..=50 {
        let key: String = keysmith::gen_key(64);
        println!("{}", key);
    }
}
