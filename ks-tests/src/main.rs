use keysmith;

fn main() {
    for _n in 1..=50 {
        println!("{}", keysmith::gen_key(64));
    }
}
