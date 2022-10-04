pub fn test() -> i32 {
    8
}

pub fn gen_key() -> String {
    let eng_alphabet: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    eng_alphabet
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
