mod gen_char;
use gen_char::gen_char;

// Generates a key string. Ex: LlyqZk2W-Hm6Eoid~m(A8ymiM0q3ksyJ
pub fn gen_key(length: u32) -> String {
    let mut output = String::from("");
    for _n in 1..=length {
        let c = gen_char(
            true, // nums
            true, // letters
            true, // upper
            true, // safe_sp_ch
            false // unsafe_sp_ch
        );

        output.push(c);
    }

    output // Return output as String
}

// Generate numbers and letters (no uppercase)
fn gen_uuid_v4_char() -> char {
    gen_char(true, true, false, false, false)
}

// Generate a uuid (version 4). Ex: e54h7tmn-b8kp-hykv-kn0o-nl9aypd9kc9e
pub fn gen_uuid_v4() -> String {
    let mut output = String::from("");

    for _n in 1..=8 {
        let c = gen_uuid_v4_char();
        output.push(c);
    }

    output.push('-'); // Current state: e54h7tmn-

    for _n in 1..=3 {
        for _x in 1..=4 {
            let c = gen_uuid_v4_char();
            output.push(c);
        }

        output.push('-');
        // After one loop: e54h7tmn-b8kp-
    }

    // e54h7tmn-b8kp-hykv-kn0o-

    for _n in 1..=12 {
        let c = gen_uuid_v4_char();
        output.push(c);
    }

    output // e54h7tmn-b8kp-hykv-kn0o-nl9aypd9kc9e
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests that the generated key is the correct length
    #[test]
    fn test_length() {
        let length = 32;
        let key = gen_key(length);

        assert_eq!(key.len(), 32);
    }
}
