use rand::Rng;

// Generates a key string
pub fn gen_key(length: u32) -> String {
    // Gen options. Will be optional args later.
    let nums = true;
    let letters = true;
    let s_sp_ch = true;
    let u_sp_ch = false;

    // Usable characters
    let numbers = "0123456789";
    let eng_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let safe_sp_chars = "-_.()~@";
    let unsafe_sp_chars = r#"#%&*+={}\/<>?!$:'"`|"#;

    // See this https://stackoverflow.com/a/40415059
    // Might want to look more into this later

    let mut chars = String::from("");

    if nums {
        chars.push_str(numbers);
    }

    if letters {
        chars.push_str(eng_alphabet);
    }

    if s_sp_ch {
        chars.push_str(safe_sp_chars);
    }

    if u_sp_ch {
        chars.push_str(unsafe_sp_chars);
    }

    let mut output = String::from("");

    for _n in 1..=length {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..chars.len());
        let c = chars.chars().nth(idx).unwrap();

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
