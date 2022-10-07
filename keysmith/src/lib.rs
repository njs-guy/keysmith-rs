mod possible_chars;
use possible_chars::get_poss_chars;
use rand::Rng;

// Generates a key string
pub fn gen_key(length: u32) -> String {
    // gen_key options. Will be optional args later.
    let nums = true;
    let letters = true;
    let s_sp_ch = true;
    let u_sp_ch = false;

    let possible_chars = get_poss_chars();

    let mut chars = String::from("");

    // Set allowed characters
    if nums {
        chars.push_str(
            possible_chars.get("numbers")
            .expect("Could not convert numbers in possible_chars Hashmap.")
        );
    }

    if letters {
        chars.push_str(
            possible_chars.get("en_alphabet")
            .expect("Could not convert en_alphabet in possible_chars Hashmap.")
        );
    }

    if s_sp_ch {
        chars.push_str(
            possible_chars.get("safe_sp_chars")
            .expect("Could not convert safe_sp_chars in possible_chars Hashmap.")
        );
    }

    if u_sp_ch {
        chars.push_str(
            possible_chars.get("unsafe_sp_chars")
            .expect("Could not convert unsafe_sp_chars in possible_chars Hashmap.")
        );
    }

    let mut output = String::from("");

    for _n in 1..=length {
        let mut rng = rand::thread_rng();

        // get a rand index from chars
        let idx = rng.gen_range(0..chars.len());

        // get the value of the index
        let c = chars.chars().nth(idx).unwrap(); 

        // push the char to output
        output.push(c);
    }

    output // Return output as String
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    // Tests for a valid file name
    fn valid_file_name() {
        println!("valid_file_name");
    }
}
