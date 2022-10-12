use keysmith;

fn main() {
    for _n in 1..=10 {
        let key: String = keysmith::gen_key(64);
        println!("{}", key);
    }
}
