mod gen_char;
use gen_char::gen_char;

// Generates a key string
pub fn gen_key(length: u32) -> String {
    let mut output = String::from("");
    for _n in 1..length {
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
