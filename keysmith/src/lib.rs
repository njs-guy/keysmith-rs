use rand::Rng;

pub fn test() -> i32 {
    8
}

// Returns a random u8 from 0 to 9
fn rand_digit() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=9)
}

// Generates a key string
pub fn gen_key(length: u32) -> String {
    // Gen options. Will be optional args later.
    let nums = true;
    let letters = true;
    let s_sp_ch = true;
    let u_sp_ch = false;

    // Usable characters
    let eng_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let safe_sp_chars = "-_.()~@";
    let unsafe_sp_chars = r#"#%&*+={}\/<>?!$:'"`|"#;

    // See this https://stackoverflow.com/a/40415059
    // Might want to look more into this later

    let mut chars = String::from("");

    if nums {
        chars.push('0');
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

        let result:String;

        match c {
            '0' => {
                result = rand_digit().to_string();
            },
            _=> {
                result = c.to_string();
            }
        }

        output.push_str(&result);
    }

    output // Return output as String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Tests that rand_digit does not generate a u8 higher than 9.
    fn digit_range() {
        let mut x: u8;
        let max = 9;

        for _n in 1..=50 {
            x = rand_digit();

            if x > max {
                panic!("rand_digit generated a number higher than {}. Got {}.", 
                max, x);
            }
        }
    }

    #[test]
    // Tests for a valid file name
    fn valid_file_name() {
        println!("valid_file_name");
    }
}
